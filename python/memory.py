from typing import Dict, Any
from interface import Proposition, Concept, Intrinsic

def dict_to_json(d: Dict[str, Any]) -> Dict[str, str]:
    return {k: str(v) for k, v in d.items()}

class Memory:
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
