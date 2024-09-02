from typing import Dict, List
from interface import Expression, ObjAttrExp, Proposition, Knowledge, ConstData

class ObjectModel:
    obj_type: str
    attr: Dict[str, ObjAttrExp]
    # TODO, relations between obj's attributes
    conclusion_about_attr: Dict[str, Proposition]
    obj_lib: List[str]
    attr_data: Dict[str, List[ConstData]]
    general: Knowledge

    def __init__(self, obj_type: str, general: Knowledge):
        self.obj_type = obj_type
        self.attr = {}
        self.conclusion_about_attr = {}
        self.general = general

    def register_objattrexp(self, objattrexp: ObjAttrExp) -> str | None:
        name = self.general.register_expr(Expression.ObjAttrExp(objattrexp))
        if name is not None:
            print(f"\033[1m" + f"Registered New Onebody Intrinsic Concept: {name} = {objattrexp}" + f"\033[0m")
            self.attr[name] = objattrexp
        return name
