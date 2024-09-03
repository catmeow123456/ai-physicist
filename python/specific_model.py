import sympy as sp
from typing import List, Tuple, Dict, Set
from interface import (
    Knowledge, ExpData,
    ExpStructure, Exp, AtomExp, Proposition, MeasureType,
    is_conserved_const_list
)
from memory import Memory
from diffalg.diffalg import DifferentialRing, diffalg


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
    conserved_list: List[Tuple[str, Exp]]
    zero_list: List[Tuple[str, Exp]]

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
        self.conserved_list = []
        self.zero_list = []

    def exp_hashed(self, exp: Exp):
        return self.general.K.eval_exp_keyvaluehashed(exp)

    # 待修改（下面的所有函数都处于最 naive 的实现，之后需要添加更多的逻辑来进行优化）

    def pick_relevant_exprs(self) -> ExpStructure:
        """
        这个函数的目的是选取当前实验中的一些 specific 的原子表达式 （ 例如 posx[1], v[2] 等等 ） 。
        这些 specific 的原子表达式由概念库中的概念 specialize 生成，以备后续组合出更复杂的表达式。
        TODO：需要有方向性的智能的随机选取，且这种随机选取方式是可学习的
        """
        for key in self.memory.fetch_concepts():
            specific_exprs: list[AtomExp] = self.general.specialize_concept(key, self.exp_name)
            # if len(specific_exprs) > 0:
            #     print(f"specialize_concept({self.exp_name}, {key}) = {specific_exprs}")
            for atom_exp in specific_exprs:
                self.general.eval(Exp.Atom(atom_exp), self.experiment)
                # 在这个 eval 过程中，
                # atom_exp 的计算结果会自动被记录到 self.experiment.data_info() 中
        for key in self.memory.fetch_objattrexps():
            specific_exprs: list[AtomExp] = self.general.specialize_concept(key, self.exp_name)
            for atom_exp in specific_exprs:
                self.general.eval(Exp.Atom(atom_exp), self.experiment)
        return self.experiment.data_info()

    def append_conserved_exp(self, conserved_exp: Exp) -> str:
        hashed_value = self.exp_hashed(conserved_exp)
        if hashed_value.is_none or hashed_value.is_const:
            # is_const 代表这个表达式是平凡的守恒量，例如 m[1] * m[2] / k[3], -1 等等
            # is_none 是极个别特殊情况，代表在计算哈希值时出现了无法计算的情况。
            return None
        # print(f"conserved exp = {conserved_exp} hashed_value = {hashed_value.get_data()}")
        for _, exp in self.conserved_list:
            if self.exp_hashed(exp) == hashed_value:
                # print(f"exp = {self.exp_hashed(exp).get_data()}, conserved_exp = {conserved_exp.exp_hashed(exp).get_data()}")
                return None
        name = self.memory.register_conclusion(Proposition.IsConserved(conserved_exp))
        self.conserved_list.append((name, conserved_exp))
        return name

    def append_zero_exp(self, zero_exp: Exp) -> str:
        hashed_value = self.exp_hashed(zero_exp)
        if hashed_value.is_none or hashed_value.is_zero:
            # is_zero 代表这个表达式是平凡的零量，例如 m[1] - m[1] 等等
            return None
        for _, exp in self.zero_list:
            if self.exp_hashed(exp) == hashed_value:
                return None
        name = self.memory.register_conclusion(Proposition.IsZero(zero_exp))
        self.zero_list.append((name, zero_exp))
        return name

    def conclusion_raw_complexity(self, prop: Proposition) -> int:
        """
        这个函数的目的是计算一个结论（ conclusion ）的 rawdefinition 的复杂度，以便在 reduce_conclusions 函数中进行排序
        """
        return self.general.K.raw_definition_prop(prop).get_complexity()

    def reduce_conclusions(self, debug=False):
        """
        这个函数的目的是将当前实验中的所有的 conserved 和 zero 的表达式整理并取 minimal 表示
        """
        conclusions: Dict[str, Proposition] = self.memory.fetch_conclusions()
        name_list: List[str] = list(conclusions.keys())
        name_list = sorted(name_list, key=lambda x: self.conclusion_raw_complexity(conclusions[x]))
        # 第一步：提取 DifferentialRing
        all_symbols = set()
        all_functions = set()
        for name, _ in self.conserved_list:
            all_symbols.add(sp.Symbol(name))
        for value in conclusions.values():
            all_symbols |= self._sympy_of_raw_defi(value.unwrap_exp).atoms(sp.Symbol)
            all_functions |= self._sympy_of_raw_defi(value.unwrap_exp).atoms(sp.Function)
        argument = sp.Symbol("t_0")
        if all_symbols.__contains__(argument):
            all_symbols.remove(argument)
        ring = DifferentialRing([('lex', list(all_functions)),
                                 ('lex', list(all_symbols))])
        # 第二步：TODO 把无意义的 conclusion 去掉
        ideal: diffalg = diffalg(ring)
        if debug:
            print('prepare ring', list(all_symbols) + list(all_functions))
        new_name_list = []
        for name in name_list:
            prop = conclusions[name]
            sp_expr = sp.simplify(self._sympy_of_raw_defi(prop.unwrap_exp))
            if prop.prop_type == "IsConserved":
                new_eq = sp.diff(sp_expr, argument).as_numer_denom()[0]
                if ideal.belongs_to(new_eq):
                    eq_reduced = ideal.gb[0].reduce(sp_expr)
                    if eq_reduced.diff(argument).is_zero:
                        # if eq_reduced is composed by all const value, then remove it
                        self.memory.remove_conclusion(name)
                    else:
                        print(prop.unwrap_exp, '-->', sp_expr, ' --eq_reduced--> ', eq_reduced)
                        new_eq = sp_expr - sp.Symbol(name)
                        if debug:
                            print('add new eq to ideal', new_eq)
                        ideal = ideal._insert_new_eq(new_eq)
                        new_name_list.append(name)
                else:
                    new_eq = sp_expr - sp.Symbol(name)
                    if debug:
                        print('add new eq to ideal', new_eq)
                    ideal = ideal._insert_new_eq(new_eq)
                    new_name_list.append(name)
            elif prop.prop_type == "IsZero":
                new_eq = sp_expr.as_numer_denom()[0]
                if ideal.belongs_to(new_eq):
                    self.memory.remove_conclusion(name)
                else:
                    if debug:
                        print('add new eq to ideal', sp_expr)
                    ideal = ideal._insert_new_eq(sp_expr)
                    new_name_list.append(name)
        # 最后一步：更新 conserved_list 和 zero_list
        self.conserved_list = []
        self.zero_list = []
        for name in new_name_list:
            prop = conclusions[name]
            if prop.prop_type == "IsConserved":
                self.conserved_list.append((name, prop.unwrap_exp))
            elif prop.prop_type == "IsZero":
                self.zero_list.append((name, prop.unwrap_exp))
        pass

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
        expdata_list = [expdata.const_data()]
        for new_exp in self.experiment_control[-1]:
            new_expdata = self.general.eval(exp, new_exp)
            if new_expdata.is_const:
                expdata_list.append(new_expdata.const_data())
            else:
                return False, None
        if not is_conserved_const_list(expdata_list):
            return False, None

        relevant_ids = set()

        ids = self.experiment.get_all_ids()
        for id in ids:
            if not self.experiment_control.__contains__(id):
                self.experiment_control[id] = []
                for _ in range(5):
                    new_exp = self.experiment.copy()
                    new_exp.random_set_obj(id)
                    new_exp.collect_expdata(MeasureType.default())
                    self.experiment_control[id].append(new_exp)
            expdata_list = [expdata.const_data()]
            for new_exp in self.experiment_control[id]:
                new_expdata = self.general.eval(exp, new_exp)
                if new_expdata.is_const:
                    expdata_list.append(new_expdata.const_data())
                else:
                    return False, None
            if not is_conserved_const_list(expdata_list):
                relevant_ids.add(id)

        return True, relevant_ids


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
        for name, exp in self.zero_list:
            print(name, "zero:", exp, "=", self._sympy_of_raw_defi(exp))
        for name, exp in self.conserved_list:
            print(name, "conserved:", exp, "=", self._sympy_of_raw_defi(exp))
    def list_sympy_conclusion(self) -> List[Tuple[str, sp.Expr]]:
        res = []
        for name, exp in self.zero_list:
            res.append((name, self._sympy_of_raw_defi(exp)))
        for name, exp in self.conserved_list:
            res.append((name, self._sympy_of_raw_defi(exp)))
        return res
