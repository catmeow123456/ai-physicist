from typing import Dict, List
from interface import Proposition, Concept, Intrinsic

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
