#%%
from typing import List, Dict
from ai_physicist import (
    Exp,
    SExp,
    TExp,
    AtomExp,
    IExpConfig,
    Knowledge,      # .fetch_concepts()  .list_concepts()  .list_experiments()
                    # .get_expstruct_pure(name: str)
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
class Theorist:
    K: Knowledge
    id: int = 0
    def __init__(self):
        self.K = Knowledge()
    def fetch_exps(self) -> List[str]:
        return self.K.fetch_experiments()
    def fetch_concepts(self) -> Dict[str, Expression]:
        return self.K.fetch_concepts()
    def fetch_expstruct(self, name: str) -> ExpStructure:
        return self.K.get_expstruct_pure(name)
    def eval(self, expr: str, expstruct: ExpStructure) -> ExpData:
        return self.K.eval(sentence.parse_exp(expr), expstruct)

    def register_expr(self, name: str, definition: str):
        expr = sentence.parse(definition)
        self.K.register_expression(name, expr)
    def auto_name(self) -> str:
        self.id += 1
        return "C_{:02d}".format(self.id)

    def generalize(self, exp_name: str, exp: str) -> Expression:
        return Expression.TExp(self.K.generalize(sentence.parse_exp(exp), exp_name))
    def specialize(self, texp: str, exp_name: str) -> List[Expression]:
        return self.K.specialize(sentence.parse_texp(texp), exp_name)
    def fetch_concept_texp(self, concept_name: str) -> TExp:
        return self.K.fetch_concept_by_name(concept_name).unwrap_texp()
    def specialize_concept(self, concept_name: str, exp_name: str) -> List[AtomExp]:
        return self.K.specialize_concept(concept_name, exp_name)
    def print_concepts(self):
        self.K.list_concepts()

# %%
