from interface import Theorist
from interface import search_relations, ExpStructure, MeasureType, Exp, AtomExp, ExpData, DataStruct, Expression

# 一个非常简餐粗暴的函数（用于测试，详见 test9.py）
# 将一个理论家记忆中的所有概念实例化（specialize）到一个实验中的具体表达式
# 再对具体表达式进行各种加减乘除求导的拼凑组合求值，
# 如果结果守恒，就将这个表达式注册为新的概念（generalize）
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
