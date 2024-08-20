from typing import Dict
from interface import Expression, ObjAttrExp, Proposition

class ObjectModel:
    obj_type: str
    attr: Dict[str, ObjAttrExp]
    # TODO, relations between obj's attributes
    conclusion_about_attr: Dict[str, Proposition]

    def __init__(self, obj_type: str):
        self.obj_type = obj_type
        self.attr = {}
        self.conclusion_about_attr = {}

    def register_objattrexp(name: str, objattrexp: ObjAttrExp):
        self.attr[name] = objattrexp
