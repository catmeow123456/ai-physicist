import sympy as sp
from typing import List, Tuple, Dict
from interface import (
    Knowledge,
    ExpStructure, Exp, AtomExp, Proposition, MeasureType
)

class SpecificModel:
    exp_name: str
    knowledge: Knowledge
    experiment: ExpStructure
    conserved_list: List[Tuple[str, Exp]]
    zero_list: List[Tuple[str, Exp]]

    def __init__(self, exp_name: str, exp_struct: ExpStructure):
        self.exp_name = exp_name
        self.knowledge = Knowledge.empty()
        self.knowledge.register_expstruct(exp_name, exp_struct)
        self.experiment = self.knowledge.fetch_expstruct(exp_name)
        self.experiment.random_settings()
        self.experiment.collect_expdata(MeasureType.default())
        self.conserved_list = []
        self.zero_list = []

    def exp_hashed(self, exp: Exp):
        return self.knowledge.K.eval_exp_keyvaluehashed(exp)

    # 待修改（下面的所有函数都处于最 naive 的实现，之后需要添加更多的逻辑来进行优化）

    def pick_relevant_exprs(self) -> ExpStructure:
        """
        这个函数的目的是选取当前实验中的一些 specific 的原子表达式 （ 例如 posx[1], v[2] 等等 ） 。
        这些 specific 的原子表达式由概念库中的概念 specialize 生成，以备后续组合出更复杂的表达式。
        TODO：需要有方向性的智能的随机选取，且这种随机选取方式是可学习的
        """
        for key in self.knowledge.fetch_concepts():
            specific_exprs: list[AtomExp] = self.knowledge.specialize_concept(key, self.exp_name)
            # if len(specific_exprs) > 0:
            #     print(f"specialize_concept({self.exp_name}, {key}) = {specific_exprs}")
            for atom_exp in specific_exprs:
                self.knowledge.eval(str(atom_exp), self.experiment)
                # 在这个 eval 过程中，
                # atom_exp 的计算结果会自动被记录到 self.experiment.data_info() 中
        return self.experiment.data_info()

    def append_conserved_exp(self, conserved_exp: Exp) -> str:
        hashed_value = self.exp_hashed(conserved_exp)
        if hashed_value.is_none or hashed_value.is_const:
            return None
        # print(f"conserved exp = {conserved_exp} hashed_value = {hashed_value.get_data()}")
        for _, exp in self.conserved_list:
            if self.exp_hashed(exp) == hashed_value:
                # print(f"exp = {self.exp_hashed(exp).get_data()}, conserved_exp = {conserved_exp.exp_hashed(exp).get_data()}")
                return None
        name = self.knowledge.register_conclusion(str(Proposition.IsConserved(conserved_exp)))
        self.conserved_list.append((name, conserved_exp))
        return name

    def append_zero_exp(self, zero_exp: Exp) -> str:
        hashed_value = self.exp_hashed(zero_exp)
        for _, exp in self.zero_list:
            if self.exp_hashed(exp) == hashed_value:
                return None
        name = self.knowledge.register_conclusion(str(Proposition.IsZero(zero_exp)))
        self.zero_list.append((name, zero_exp))
        return name

    def conclusion_raw_complexity(self, prop: Proposition) -> int:
        """
        这个函数的目的是计算一个结论（ conclusion ）的 rawdefinition 的复杂度，以便在 reduce_conclusions 函数中进行排序
        """
        return self.knowledge.K.raw_definition_prop(prop).get_complexity()

    def reduce_conclusions(self):
        """
        这个函数的目的是将当前实验中的所有的 conserved 和 zero 的表达式整理并取 minimal 表示
        """
        conclusions: Dict[str, Proposition] = self.knowledge.K.fetch_conclusions()
        name_list: List[str] = list(conclusions.keys())
        name_list = sorted(name_list, key=lambda x: self.conclusion_raw_complexity(conclusions[x]))
        self.conserved_list = []
        self.zero_list = []
        for name in name_list:
            prop = conclusions[name]
            if prop.prop_type == "IsConserved":
                self.conserved_list.append((name, prop.unwrap_exp))
            elif prop.prop_type == "IsZero":
                self.zero_list.append((name, prop.unwrap_exp))
        # for name, prop in conclusions:
        #     prop_type = prop.prop_type
        #     if prop_type == "IsConserved":
        #         pass
        pass

    def print_conclusion(self):
        print(f"Exp's name = {self.exp_name}, conclusions:")
        self.knowledge.print_conclusions()

    def print_full_conclusion(self):
        for name, exp in self.conserved_list:
            print(name, "conserved:", exp, "=", self.knowledge.K.raw_definition_exp(exp))
        for name, exp in self.zero_list:
            print(name, "zero:", exp, "=", self.knowledge.K.raw_definition_exp(exp))

    def _sympy_of_raw_defi(self, exp: Exp) -> sp.Expr:
        return sp.sympify(self.knowledge.K.parse_exp_to_sympy_str(
            self.knowledge.K.raw_definition_exp(exp),
            "t_0"
        ))

    def print_sympy_conclusion(self):
        for name, exp in self.conserved_list:
            print(name, "conserved:", exp, "=", self._sympy_of_raw_defi(exp))
        for name, exp in self.zero_list:
            print(name, "zero:", exp, "=", self._sympy_of_raw_defi(exp))
    def list_sympy_conclusion(self) -> List[Tuple[str, sp.Expr]]:
        res = []
        for name, exp in self.conserved_list:
            res.append((name, self._sympy_of_raw_defi(exp)))
        for name, exp in self.zero_list:
            res.append((name, self._sympy_of_raw_defi(exp)))
        return res
