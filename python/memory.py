from typing import Dict, List
from interface import Proposition, TExp, ObjAttrExp

class Memory:
    concept: Dict[str, TExp]
    objattrexp: Dict[str, ObjAttrExp]
    conclusion: Dict[str, Proposition]
    conclusion_id: int

    def __init__(self):
        self.concept = {}
        self.objattrexp = {}
        self.conclusion = {}
        self.conclusion_id = 0

    def register_concept(self, texp: TExp, name: str):
        self.concept[name] = texp

    def register_objattrexp(self, objattrexp: ObjAttrExp, name: str):
        self.objattrexp[name] = objattrexp

    def register_conclusion(self, prop: Proposition):
        self.conclusion_id += 1
        name = f"P{self.conclusion_id}"
        self.conclusion[name] = prop
        return name

    def remove_conclusion(self, name: str):
        if name in self.conclusion:
            del self.conclusion[name]

    def fetch_concepts(self):
        return self.concept.keys()

    def fetch_objattrexps(self):
        return self.objattrexp.keys()

    def fetch_conclusions(self) -> Dict[str, Proposition]:
        return self.conclusion
