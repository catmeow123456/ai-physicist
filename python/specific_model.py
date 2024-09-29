import sympy as sp

from tqdm import tqdm
from typing import List, Tuple, Dict, Set, Any
from interface import (
    Knowledge, ExpData, DataStruct,
    ExpStructure, Exp, AtomExp, Proposition, MeasureType,
    is_conserved_const_list
)
from memory import Memory
from diffalg.diffalg import DifferentialRing, diffalg


class ZeroInfo:
    """
    ZeroInfo 类是一个用来存储零量的信息的类
    """
    exp_name: str
    name: str
    exp: Exp

    def __str__(self):
        return f"{self.name}: {self.exp}"

    def from_json(data: Tuple[str, str, str]) -> "ZeroInfo":
        obj = object.__new__(ZeroInfo)
        obj.exp_name = data[0]
        obj.name = data[1]
        obj.exp = Exp(data[2])
        return obj

    def to_json(self) -> Tuple[str, str, str]:
        return self.exp_name, self.name, str(self.exp)


class ConservedInfo:
    """
    ConservedInfo 类是一个用来存储守恒量的信息的类，
    这些信息包括它的取值是否是内禀的，以及它依赖的实验对象编号
    """
    exp_name: str
    name: str
    exp: Exp
    is_intrinsic: bool
    relevant_id: Set[int]

    def __str__(self):
        return f"{self.name}: {self.exp}"

    def from_json(data: Tuple[str, str, str, bool, List[int]]) -> "ConservedInfo":
        obj = object.__new__(ConservedInfo)
        obj.exp_name = data[0]
        obj.name = data[1]
        obj.exp = Exp(data[2])
        obj.is_intrinsic = data[3]
        obj.relevant_id = set(data[4]) if obj.is_intrinsic else None
        return obj

    def to_json(self) -> Tuple[str, str, str, bool, List[int]]:
        res = (
            self.exp_name, self.name, str(self.exp),
            self.is_intrinsic, list(self.relevant_id) if self.relevant_id is not None else []
        )
        return res


class SpecificModel:
    """
    SpecificModel 类是专注于特定实验的物理学家模型，
    它包括一个 knowledge 对象，存储了有关这个实验的知识；
    一个 experiment 对象，存储了这个实验的具体信息和某个随机参数下的实验结果；
    以及一个 experiment_control 字典，是相对于 experiment 的实验对照组，表达了在控制变量的条件下做实验获得的新结果。
    """
    exp_name: str
    general: Knowledge
    memory: Memory
    experiment: ExpStructure
    experiment_control: Dict[int, List[ExpStructure]]
    # 保持其他实验对象不变，改变实验对象 id 并进行实验获得的结果存储在 experiment_control[id] 中
    # id = -1 代表保持所有实验对象不变，只改变实验控制参数
    conserved_list: Dict[str, ConservedInfo]
    zero_list: Dict[str, ZeroInfo]
    intrinsic_buffer: Dict[str, ConservedInfo]
    # 保证 conserved_list 与 zero_list 对应了 memory.conclusion 中的结论

    def __init__(self, exp_name: str, general: Knowledge):
        """
        初始化一个 SpecificModel 对象，需要提供实验的名称和实验的结构
        """
        self.exp_name = exp_name
        self.general = general
        self.memory = Memory()
        self.experiment = self.general.fetch_expstruct(exp_name)
        self.experiment.random_settings()
        self.experiment.collect_expdata(MeasureType.default())
        self.experiment_control = {}

        self.conserved_list = {}
        self.zero_list = {}
        self.intrinsic_buffer = {}

    def to_json(self) -> Dict[str, str]:
        return {
            "exp_name": self.exp_name,
            "memory": self.memory.to_json(),
            "conserved_list": [
                item.to_json() for item in self.conserved_list.values()
            ],
            "zero_list": [
                item.to_json() for item in self.zero_list.values()
            ],
        }

    def load_json(self, data: Dict[str, Any]):
        self.memory = Memory.from_json(data["memory"])
        self.conserved_list = {}
        for item in data["conserved_list"]:
            info = ConservedInfo.from_json(item)
            self.conserved_list[info.name] = info
        self.zero_list = {}
        for item in data["zero_list"]:
            info = ZeroInfo.from_json(item)
            self.zero_list[info.name] = info

    def exp_hashed(self, exp: Exp):
        return self.general.K.eval_exp_keyvaluehashed(exp)

    # 待修改（下面的所有函数都处于最 naive 的实现，之后需要添加更多的逻辑来进行优化）

    def pick_relevant_exprs(self) -> DataStruct:
        """
        这个函数的目的是选取当前实验中的一些 specific 的原子表达式 （ 例如 posx[1], v[2] 等等 ） 。
        这些 specific 的原子表达式由概念库中的概念 specialize 生成，以备后续组合出更复杂的表达式。
        TODO：需要有方向性的智能的随机选取，且这种随机选取方式是可学习的
        """
        return self.memory.pick_relevant_exprs(self.experiment, self.general)

    def append_conserved_exp(self, conserved_exp: Exp) -> str:
        hashed_value = self.exp_hashed(conserved_exp)
        if hashed_value.is_none or hashed_value.is_const:
            # is_const 代表这个表达式是平凡的守恒量，例如 m[1] * m[2] / k[3], -1 等等
            # is_none 是极个别特殊情况，代表在计算哈希值时出现了无法计算的情况。
            return None
        # print(f"conserved exp = {conserved_exp} hashed_value = {hashed_value.get_data()}")
        for _, info in self.conserved_list.items():
            if self.exp_hashed(info.exp) == hashed_value:
                # print(f"exp = {self.exp_hashed(info.exp).get_data()}, conserved_exp = {conserved_exp.exp_hashed(exp).get_data()}")
                return None
        name = self.memory.register_conclusion(Proposition.IsConserved(conserved_exp))
        self.conserved_list[name] = self.make_conserved_info(name, conserved_exp)
        return name

    def append_zero_exp(self, zero_exp: Exp) -> str:
        hashed_value = self.exp_hashed(zero_exp)
        if hashed_value.is_none or hashed_value.is_zero:
            # is_zero 代表这个表达式是平凡的零量，例如 m[1] - m[1] 等等
            return None
        for _, info in self.zero_list.items():
            if self.exp_hashed(info.exp) == hashed_value:
                return None
        name = self.memory.register_conclusion(Proposition.IsZero(zero_exp))
        self.zero_list[name] = self.make_zero_info(name, zero_exp)
        return name

    def conclusion_raw_complexity(self, prop: Proposition) -> int:
        """
        这个函数的目的是计算一个结论（ conclusion ）的 rawdefinition 的复杂度，以便在 reduce_conclusions 函数中进行排序
        """
        return self.general.K.raw_definition_prop(prop).complexity

    def reduce_conclusions(self, debug=False):
        """
        这个函数的目的是将当前实验中的所有的 conserved 和 zero 的表达式整理并取 minimal 表示
        """
        conclusions: Dict[str, Proposition] = self.memory.fetch_conclusions
        name_list: List[str] = list(conclusions.keys())
        name_list = sorted(name_list, key=lambda x: self.conclusion_raw_complexity(conclusions[x]))
        # 第一步：提取 DifferentialRing
        # all_symbols = set()
        all_normal_symbols = set()
        all_intrinsic_symbols = set()
        all_functions = set()
        for name, info in self.conserved_list.items():
            if info.is_intrinsic:
                all_intrinsic_symbols.add(sp.Symbol(name))
            else:
                all_normal_symbols.add(sp.Symbol(name))
        for value in conclusions.values():
            all_intrinsic_symbols |= self._sympy_of_raw_defi(value.unwrap_exp).atoms(sp.Symbol)
            all_functions |= self._sympy_of_raw_defi(value.unwrap_exp).atoms(sp.Function)
        argument = sp.Symbol("t_0")
        if all_intrinsic_symbols.__contains__(argument):
            all_intrinsic_symbols.remove(argument)
        ring = DifferentialRing([('lex', list(all_functions)),
                                 ('lex', list(all_normal_symbols)),
                                 ('lex', list(all_intrinsic_symbols))])
        # 第二步：TODO 把无意义的 conclusion 去掉
        ideal: diffalg = diffalg(ring)
        ideal.insert_new_ineqs(argument)
        if debug:
            print('prepare ring', ring)
        def insert_to_ideal(ideal: diffalg, new_eq: sp.Expr):
            if debug:
                tqdm.write(f'add new eq to ideal {new_eq} = 0')
            return ideal._insert_new_eq(new_eq)
        def insert_to_ideal_both(ideal: diffalg, new_eq: sp.Expr, new_ineqs: sp.Expr):
            if debug:
                tqdm.write(f'add new eq to ideal {new_eq} = 0 and {new_ineqs} <> 0')
            return ideal._insert_new_eqs_and_ineqs([new_eq], [new_ineqs])
        subs_dict = dict()
        inverse_dict = dict()
        for name in tqdm(name_list):
            prop: Proposition = conclusions[name]
            sp_expr = sp.simplify(
                self._sympy_of_raw_defi(prop.unwrap_exp)
                .subs(subs_dict, simultaneous=True)
            ).subs(inverse_dict, simultaneous=True)
            if_print = False
            if prop.prop_type == "IsConserved":
                if sp_expr.is_Function:
                    subs_dict[sp_expr] = sp.Symbol(sp_expr.name)
                    inverse_dict[sp.Symbol(sp_expr.name)] = sp_expr
                info: ConservedInfo = self.conserved_list.get(name)
                flag = info.is_intrinsic
                diff_eq = sp.diff(sp_expr, argument).as_numer_denom()[0]
                reduce_diff_eq_result: sp.Expr = ideal.gb[0].reduce(diff_eq)
                if reduce_diff_eq_result.is_zero:
                    eq_reduced = ideal.reduce(sp_expr)
                    if eq_reduced.diff(argument).is_zero:
                        # if eq_reduced is composed by all const value, then remove it
                        if info.is_intrinsic:
                            symbs = eq_reduced.atoms(sp.Symbol)
                            if symbs.issubset(all_intrinsic_symbols):
                                flag = False
                            else:
                                ideal = insert_to_ideal_both(ideal, sp_expr - sp.Symbol(name), sp.Symbol(name))
                                if_print = True
                        self.memory.remove_conclusion(name)
                        del self.conserved_list[name]
                    else:
                        ideal = insert_to_ideal_both(ideal, sp_expr - sp.Symbol(name), sp.Symbol(name))
                        if_print = True
                else:
                    ideal = insert_to_ideal_both(ideal, sp_expr - sp.Symbol(name), sp.Symbol(name))
                    if_print = True
                if flag:
                    self.intrinsic_buffer[name] = info
            elif prop.prop_type == "IsZero":
                new_eq = sp_expr.as_numer_denom()[0]
                if ideal.belongs_to(new_eq):
                    self.memory.remove_conclusion(name)
                    del self.zero_list[name]
                else:
                    if new_eq.func == sp.Add and len(new_eq.args) == 2:
                        atom1 = new_eq.args[0]
                        atom2 = new_eq.args[1]
                        def is_neg_symbol(x: sp.Expr):
                            return x.func == sp.Mul and x.args[0] == -1 and x.args[1].is_Symbol
                        if is_neg_symbol(atom2):
                            atom2 = -atom2
                        else:
                            atom1 = -atom1
                        subs_dict[atom2] = atom1
                    ideal = insert_to_ideal(ideal, sp_expr)
                    if_print = True
            if if_print:
                tqdm.write(f'Insert to ideal: {prop.unwrap_exp}')

    def check_intrinsic(self, exp: Exp) -> Tuple[bool, Set[int] | None]:
        """
        这个函数的目的是检查一个表达式是否是内禀概念（取值仅依赖于实验对象）
        如果是，返回 True 和它依赖的实验对象编号
        否则，返回 False 和 None
        """
        expdata: ExpData = self.general.eval(exp, self.experiment)
        if not expdata.is_const:
            return False, None
        if not self.experiment_control.__contains__(-1):
            self.experiment_control[-1] = []
            for _ in range(5):
                new_exp = self.experiment.copy()
                new_exp.random_set_exp_para()
                new_exp.collect_expdata(MeasureType.default())
                self.experiment_control[-1].append(new_exp)
        expdata_list = [expdata.const_data]
        for new_exp in self.experiment_control[-1]:
            new_expdata = self.general.eval(exp, new_exp)
            if new_expdata.is_const:
                expdata_list.append(new_expdata.const_data)
            else:
                return False, None
        if not is_conserved_const_list(expdata_list):
            return False, None

        relevant_ids = set()

        ids = self.experiment.all_ids
        for id in ids:
            if not self.experiment_control.__contains__(id):
                self.experiment_control[id] = []
                for _ in range(5):
                    new_exp = self.experiment.copy()
                    new_exp.random_set_obj(id)
                    new_exp.collect_expdata(MeasureType.default())
                    self.experiment_control[id].append(new_exp)
            expdata_list = [expdata.const_data]
            for new_exp in self.experiment_control[id]:
                new_expdata = self.general.eval(exp, new_exp)
                if new_expdata.is_const:
                    expdata_list.append(new_expdata.const_data)
                else:
                    return False, None
            if not is_conserved_const_list(expdata_list):
                relevant_ids.add(id)

        return True, relevant_ids

    def make_zero_info(self, name: str, exp: Exp) -> ZeroInfo:
        obj = object.__new__(ZeroInfo)
        obj.exp_name = self.exp_name
        obj.name = name
        obj.exp = exp
        return obj

    def make_conserved_info(self, name: str, exp: Exp) -> ConservedInfo:
        obj = object.__new__(ConservedInfo)
        obj.exp_name = self.exp_name
        obj.name = name
        obj.exp = exp
        obj.is_intrinsic, obj.relevant_id = self.check_intrinsic(exp)
        return obj

    def print_conclusion(self):
        print(f"Exp's name = {self.exp_name}, conclusions:")
        self.memory.print_conclusions()

    def print_full_conclusion(self):
        for name, exp in self.zero_list:
            print(name, "zero:", exp, "=", self.general.raw_definition_exp(exp))
        for name, exp in self.conserved_list:
            print(name, "conserved:", exp, "=", self.general.raw_definition_exp(exp))

    def _sympy_of_raw_defi(self, exp: Exp) -> sp.Expr:
        return sp.sympify(self.general.K.parse_exp_to_sympy_str(
            self.general.K.raw_definition_exp(exp),
            "t_0"
        ))

    def print_sympy_conclusion(self):
        for name, info in self.zero_list.items():
            print(name, "zero:", info.exp, "=", self._sympy_of_raw_defi(info.exp))
        for name, info in self.conserved_list.items():
            print(name, "conserved:", info.exp, "=", self._sympy_of_raw_defi(info.exp))
    def list_sympy_conclusion(self) -> List[Tuple[str, sp.Expr]]:
        res = []
        for name, info in self.zero_list.items():
            res.append((name, self._sympy_of_raw_defi(info.exp)))
        for name, info in self.conserved_list.items():
            res.append((name, self._sympy_of_raw_defi(info.exp)))
        return res
