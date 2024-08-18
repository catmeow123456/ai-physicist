from typing import List, Tuple
from interface import Knowledge, ExpStructure, Exp, AtomExp, Proposition, MeasureType

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

    def print_conclusion(self):
        print(f"Exp's name = {self.exp_name}, conclusions:")
        self.knowledge.print_conclusions()

    def print_full_conclusion(self):
        for name, exp in self.conserved_list:
            print(name, "conserved:", exp, "=", self.knowledge.K.raw_definition(exp))
        for name, exp in self.zero_list:
            print(name, "zero:", exp, "=", self.knowledge.K.raw_definition(exp))
