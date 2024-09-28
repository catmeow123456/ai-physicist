from typing import Dict, Any
from interface import (
    AtomExp, Exp, Knowledge,
    Proposition, Concept, Intrinsic, DataStruct, ExpStructure
)

def dict_to_json(d: Dict[str, Any]) -> Dict[str, str]:
    return {k: str(v) for k, v in d.items()}

class Memory:
    """
    AI 的记忆仓库，其中包括了各种概念或者其他抽象的表达，
    且每一个概念拥有一个权重（ TODO ）。
    给记忆仓库一个 pick_relevant_exprs 指令，并传入特定实验 experiment、Knowledge，
    它会根据记忆联想到与实验 experiment 相关的一些特定的原子表达式。
    后续会接入神经网络来调节这一部分。
    """
    concept: Dict[str, Concept]
    intrinsic: Dict[str, Intrinsic]
    conclusion: Dict[str, Proposition]
    conclusion_id: int

    def __init__(self):
        self.concept = {}
        self.intrinsic = {}
        self.conclusion = {}
        self.conclusion_id = 0

    def to_json(self):
        return {
            "concept": dict_to_json(self.concept),
            "intrinsic": dict_to_json(self.intrinsic),
            "conclusion": dict_to_json(self.conclusion),
            "conclusion_id": self.conclusion_id
        }

    def from_json(data: Dict[str, Any]) -> "Memory":
        obj = object.__new__(Memory)
        obj.concept = {k: Concept(v) for k, v in data["concept"].items()}
        obj.intrinsic = {k: Intrinsic(v) for k, v in data["intrinsic"].items()}
        obj.conclusion = {k: Proposition(v) for k, v in data["conclusion"].items()}
        obj.conclusion_id = data["conclusion_id"]
        return obj

    def register_concept(self, concept: Concept, name: str):
        self.concept[name] = concept

    def register_intrinsic(self, intrinsic: Intrinsic, name: str):
        self.intrinsic[name] = intrinsic

    def register_conclusion(self, prop: Proposition):
        self.conclusion_id += 1
        name = f"P{self.conclusion_id}"
        self.conclusion[name] = prop
        return name

    def remove_conclusion(self, name: str):
        if name in self.conclusion:
            del self.conclusion[name]

    @property
    def fetch_concepts(self):
        return self.concept.keys()

    @property
    def fetch_intrinsics(self):
        return self.intrinsic.keys()

    @property
    def fetch_conclusions(self) -> Dict[str, Proposition]:
        return self.conclusion

    def pick_relevant_exprs(self, experiment: ExpStructure, knowledge: Knowledge) -> DataStruct:
        """
        根据记忆联想到与 experiment 相关的一些表达式
        """
        DS = DataStruct.empty()
        for atom_exp in experiment.original_data:
            DS.add_data(atom_exp, knowledge.eval(Exp.Atom(atom_exp), experiment))
        for key in self.fetch_concepts:
            specific_exprs: list[AtomExp] = knowledge.specialize_concept(key, experiment.exp_name)
            for atom_exp in specific_exprs:
                DS.add_data(atom_exp,
                            knowledge.eval(Exp.Atom(atom_exp), experiment))
        for key in self.fetch_intrinsics:
            specific_exprs: list[AtomExp] = knowledge.specialize_concept(key, experiment.exp_name)
            for atom_exp in specific_exprs:
                DS.add_data(atom_exp,
                            knowledge.eval(Exp.Atom(atom_exp), experiment))
        # print('DataKeys:',[str(i) for i in DS.data_keys])
        return DS
