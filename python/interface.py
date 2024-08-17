#%%
from typing import List, Dict
import ai_physicist as aiphy
# aiphy.Knowledge: class
#   .fetch_concepts()  .list_concepts()  .list_experiments()
#   .get_expstruct_pure(name: str)
from ai_physicist import (
    Proposition,
    Exp,
    SExp,
    TExp,
    AtomExp,
    IExpConfig,
    Expression,
    DataStruct,
    ExpData,
    ExpStructure,
    sentence,       # .parse()  .parse_exp()  .parse_sexp()
    search_relations
)
from ai_physicist import (
    MeasureType,    # .default()
    Objstructure    # .make_masspoint() .make_spring()
)
# %%
class Knowledge:
    K: aiphy.Knowledge
    concept_id: int = 0
    conclusion_id: int = 0
    def default() -> "Knowledge":
        obj = object.__new__(Knowledge)
        obj.K = aiphy.Knowledge.default()
        return obj
    def empty() -> "Knowledge":
        obj = object.__new__(Knowledge)
        obj.K = aiphy.Knowledge()
        return obj
    def fetch_exps(self) -> List[str]:
        return self.K.fetch_experiments()
    def fetch_concepts(self) -> Dict[str, Expression]:
        return self.K.fetch_concepts()
    def register_expstruct(self, name: str, expstruct: ExpStructure):
        self.K.register_experiment(name, expstruct)
    def fetch_expstruct(self, name: str) -> ExpStructure:
        return self.K.get_expstruct_pure(name)
    def eval(self, expr: str, expstruct: ExpStructure) -> ExpData:
        return self.K.eval(sentence.parse_exp(expr), expstruct)

    def register_expr(self, definition: str, name: str = None) -> str:
        if name is None:
            name = self.auto_concept_name()
        expr: Expression = sentence.parse(definition)
        self.K.register_expression(name, expr)
        return name
    def register_conclusion(self, definition: str, name: str = None) -> str:
        if name is None:
            name = self.auto_conclusion_name()
        prop: Proposition = sentence.parse_proposition(definition)
        self.K.register_conclusion(name, prop)
        return name
    def auto_concept_name(self) -> str:
        self.concept_id += 1
        return "C_{:02d}".format(self.concept_id)
    def auto_conclusion_name(self) -> str:
        self.conclusion_id += 1
        return "R_{:02d}".format(self.conclusion_id)

    def generalize(self, exp_name: str, exp: str) -> Expression:
        try:
            return Expression.TExp(self.K.generalize(sentence.parse_exp(exp), exp_name))
        except:
            print("Failed to generalize", exp_name, exp)
    def specialize(self, texp: str, exp_name: str) -> List[Expression]:
        return self.K.specialize(sentence.parse_texp(texp), exp_name)
    def fetch_concept_texp(self, concept_name: str) -> TExp:
        return self.K.fetch_concept_by_name(concept_name).unwrap_texp()
    def specialize_concept(self, concept_name: str, exp_name: str) -> List[AtomExp]:
        return self.K.specialize_concept(concept_name, exp_name)
    def print_concepts(self):
        self.K.list_concepts()
    def print_conclusions(self):
        self.K.list_conclusions()

# %%
