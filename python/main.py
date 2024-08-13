from interface import Theorist
from interface import search_relations, ExpStructure, MeasureType, Exp, AtomExp, ExpData, DataStruct, Expression

def work_at_exp(theorist: Theorist, exp_name: str) -> ExpStructure:
    exp = theorist.fetch_expstruct(exp_name)
    exp.random_settings()
    exp.collect_expdata(MeasureType.default())
    for key in theorist.fetch_concepts():
        specific_exprs: list[AtomExp] = theorist.specialize_concept(key, exp_name)
        for i in specific_exprs:
            theorist.eval(str(i), exp)
    data_info: DataStruct = exp.data_info()
    print(data_info)
    res: list[tuple[Exp, ExpData]] = search_relations(data_info)
    for i in res:
        expr: Expression = theorist.generalize(exp_name, str(i[0]))
        theorist.register_expr(theorist.auto_name(), str(expr))
    return exp
