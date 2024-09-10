from typing import List, Dict
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
    sentence,       # .parse()  .parse_exp()  .parse_sexp()
    search_trivial_relations,
    search_relations,
    search_relations_ver2,
)
from ai_physicist import (
    ExpData,
    ConstData,
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

    def default() -> "Knowledge":
        obj = object.__new__(Knowledge)
        obj.K = aiphy.Knowledge.default()
        return obj

    def empty() -> "Knowledge":
        obj = object.__new__(Knowledge)
        obj.K = aiphy.Knowledge()
        return obj

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
        if isinstance(expr, str):
            expr = sentence.parse_exp(expr)
        return self.K.eval(expr, expstruct)

    def register_object(self, objstruct: Objstructure, name: str = None) -> str:
        name = self.auto_object_name() if name is None else name
        self.K.register_object(name, objstruct)
        return name
    def register_expr(self, definition: Expression | str, name: str = None) -> str | None:
        name = self.auto_concept_name() if name is None else name
        expr: Expression = sentence.parse(definition) if isinstance(definition, str) else definition
        if self.K.register_expression(name, expr):
            return name
        else:
            return None
    def register_conclusion(self, definition: str, name: str = None) -> str:
        name = self.auto_conclusion_name() if name is None else name
        prop: Proposition = sentence.parse_proposition(definition)
        self.K.register_conclusion(name, prop)
        return name
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
            exp: Exp = sentence.parse_exp(exp) if isinstance(exp, str) else exp
            return Expression.Concept(self.K.generalize(exp, exp_name))
        except:
            print("Failed to generalize", exp_name, exp)
    def specialize(self, concept: str, exp_name: str) -> List[Expression]:
        return self.K.specialize(sentence.parse_concept(concept), exp_name)
    def fetch_concept_concept(self, concept_name: str) -> Concept:
        return self.K.fetch_concept_by_name(concept_name).unwrap_concept
    def specialize_concept(self, concept_name: str, exp_name: str) -> List[AtomExp]:
        return self.K.specialize_concept(concept_name, exp_name)
    def print_concepts(self):
        self.K.list_concepts()
    def print_conclusions(self):
        self.K.list_conclusions()
