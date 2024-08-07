#%%
from typing import List, Dict
from ai_physicist import (
    SExp,
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

    def generalize(self, exp_name: str, exp: str) -> Expression:
        sexp = SExp.Mk(IExpConfig.From(exp_name), sentence.parse_exp(exp))
        return Expression.TExp(self.K.generalize(sexp))

# %%
