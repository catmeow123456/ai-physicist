from typing import List, Dict, Set
import json
import ai_physicist as aiphy
from ai_physicist import (
    Proposition,
    Exp,
    SExp,
    Concept,
    AtomExp,
    IExpConfig,
    Intrinsic,
    Expression,
    DataStruct,
    ExpStructure,
    search_trivial_relations,
    search_relations,
    search_relations_ver2,
    search_relations_ver3
)
from ai_physicist import (
    ExpData,
    ConstData,
    NormalData,
    KeyValueHashed,
    is_conserved_const_list,
)
from ai_physicist import (
    MeasureType,    # .default()
    Objstructure    # .make_masspoint() .make_spring()
)

class Knowledge:
    """
    Knowledge 类，是 ai_physicist.Knowledge （ built-in 的方法是在 rust 中实现的 ） 的一个包装类，提供了一些便捷的方法，
    存储各种知识，包括实验、概念、结论等。
    可以在这个知识库中注册和查询概念与结论。

    查看当前知识库信息的方法：    
    fetch_expstruct()  fetch_exps()  fetch_concepts()  print_concepts()  print_conclusions()
    注册新的概念、结论、实验的方法：
    register_expr()  register_conclusion()  register_expstruct()
    """
    K: aiphy.Knowledge
    concept_id: int = 0
    conclusion_id: int = 0
    object_id: int = 0

    obj_tmp: Set[str] = set()

    def default() -> "Knowledge":
        """
        创建一个新的 Knowledge 对象，
        内部包含一些默认的实验 （ 程序内置的实验 ） 。
        """
        obj = object.__new__(Knowledge)
        obj.K = aiphy.Knowledge.default()
        return obj

    def read_from_file(filename: str) -> "Knowledge":
        """
        从文件中读取知识库。
        """
        obj = object.__new__(Knowledge)
        with open(filename, "r") as f:
            s = f.read().strip()
            s1 = s[:s.find("[end]") + 5]
            s2 = s[s.find("[end]") + 5:].strip()
            try:
                obj.K = aiphy.Knowledge.from_string(s1)
            except:
                print("Failed to load knowledge from string")
                print(s1)
                raise Exception("Failed to load knowledge from string")
            id_dict = json.loads(s2)
            obj.concept_id = id_dict["concept_id"]
            obj.conclusion_id = id_dict["conclusion_id"]
            obj.object_id = id_dict["object_id"]
        for obj_name in obj.K.fetch_object_keys:
            obj.obj_tmp.add(obj_name)
        return obj

    def save_to_file(self, filename: str):
        """
        将当前知识库保存到文件中。
        """
        self.remove_useless_objects()
        print("Saving knowledge to", filename)
        with open(filename, "w") as f:
            f.write(str(self.K))
            json.dump({
                "concept_id": self.concept_id,
                "conclusion_id": self.conclusion_id,
                "object_id": self.object_id
            }, f)

    @property
    def fetch_exps(self) -> List[str]:
        return self.K.fetch_experiments
    @property
    def fetch_concepts(self) -> Dict[str, Expression]:
        return self.K.fetch_concepts
    def register_expstruct(self, name: str, expstruct: ExpStructure):
        self.K.register_experiment(name, expstruct)
    def fetch_expstruct(self, name: str) -> ExpStructure:
        return self.K.fetch_expstruct(name)

    def eval(self, expr: Exp | str, expstruct: ExpStructure) -> ExpData:
        """
        在特定的实验数据结构 （ 包含测量数据 ） 下，计算一个表达式的值。
        """
        if isinstance(expr, str):
            expr = Exp(expr)
        try:
            return self.K.eval(expr, expstruct)
        except:
            print("Failed to eval", expr)
            raise Exception("Failed to eval")

    def register_object(self, objstruct: Objstructure, name: str = None) -> str:
        """
        以 name 为名字，注册一个具体的物理对象 objstruct。
        """
        name = self.auto_object_name() if name is None else name
        self.K.register_object(name, objstruct)
        return name
    def register_expr(self, definition: Expression | str, name: str = None) -> str | None:
        """
        以 name 为名字，注册一个概念 definition ，
        1. 它可以是 Concept （普通概念）， 例如
        "(1->MassPoint) (2->Clock) |- D[posx[1]]/D[t[2]]"
        2. 或者是 Intrinsic （内禀概念）， 例如
        "[#oscillation (1->MassPoint) [2->Obj_02] |- D[posx[1]]/D[posx[1]'']]"
        """
        name = self.auto_concept_name() if name is None else name
        expr: Expression = Expression(definition) if isinstance(definition, str) else definition
        if self.K.register_expression(name, expr):
            # 概念注册成功
            if expr.expr_type == "Intrinsic":
                # 如果是内禀概念，那么将其相关的物体名字加入 obj_tmp
                intr: Intrinsic = expr.unwrap_intrinsic
                for obj_name in intr.relevant_objs:
                    self.obj_tmp.add(obj_name)
            return name
        else:
            return None
    def register_conclusion(self, definition: str, name: str = None) -> str:
        """
        以 name 为名字，注册一个结论 definition ，
        1. 它可以是 Exp(...) is zero， 例如
            "(posx[1] - posr[2]) is zero"
        2. 它可以是 Exp(...) is conserved， 例如
            "(m[1] * v[1] + m[2] * v[2]) is conserved"
        """
        name = self.auto_conclusion_name() if name is None else name
        prop: Proposition = Proposition(definition)
        self.K.register_conclusion(name, prop)
        return name
    def remove_useless_objects(self):
        """
        移除那些没有被任何内禀概念引用的物体。
        """
        for obj_name in self.K.fetch_object_keys:
            if obj_name not in self.obj_tmp:
                self.K.remove_object(obj_name)

    def auto_object_name(self) -> str:
        self.object_id += 1
        return "Obj_{:02d}".format(self.object_id)
    def auto_concept_name(self) -> str:
        self.concept_id += 1
        return "C_{:02d}".format(self.concept_id)
    def auto_conclusion_name(self) -> str:
        self.conclusion_id += 1
        return "R_{:02d}".format(self.conclusion_id)

    def generalize(self, exp_name: str, exp: Exp | str) -> Expression:
        try:
            exp: Exp = Exp(exp) if isinstance(exp, str) else exp
            return Expression.Concept(self.K.generalize(exp, exp_name))
        except:
            print("Failed to generalize", exp_name, exp)
    def specialize(self, concept: str, exp_name: str) -> List[Expression]:
        return self.K.specialize(Concept(concept), exp_name)
    def fetch_concept_by_name(self, concept_name: str) -> Concept:
        return self.K.fetch_concept_by_name(concept_name).unwrap_concept
    def specialize_concept(self, concept_name: str, exp_name: str) -> List[AtomExp]:
        return self.K.specialize_concept(concept_name, exp_name)

    def print_concepts(self):
        """
        打印当前知识库中的所有概念。
        """
        self.K.list_concepts()

    def print_conclusions(self):
        """
        打印当前知识库中的所有结论。
        """
        self.K.list_conclusions()
