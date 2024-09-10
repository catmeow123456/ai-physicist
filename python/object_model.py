from typing import Dict, List
from interface import Expression, Intrinsic, Proposition, Knowledge, ConstData
from interface import Objstructure

class ObjectModel:
    general: Knowledge
    obj_type: str
    attr: Dict[str, Intrinsic]
    # TODO, relations between obj's attributes
    conclusion_about_attr: Dict[str, Proposition]
    # TODO, 每个 attr 测量了 obj_type 物体的某个属性。但是测量是有量程的，一旦超过这个量程，测量就会失败。
    # 所以倾向于选择量程大的测量方式。在此基础上进行判重。

    def __init__(self, obj_type: str, general: Knowledge):
        self.general = general
        self.obj_type = obj_type
        self.attr = {}
        self.conclusion_about_attr = {}
        self.obj_lib = []

    def register_intrinsic(self, intrinsic: Intrinsic) -> str | None:
        name = self.general.register_expr(Expression.Intrinsic(intrinsic))
        if name is not None:
            print(f"\033[1m" + f"Registered New Onebody Intrinsic Concept: {name} = {intrinsic}" + f"\033[0m")
            self.attr[name] = intrinsic
        return name
