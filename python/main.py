from typing import Dict, List, Tuple
from specific_model import SpecificModel
from interface import Knowledge
from interface import search_relations, ExpStructure, MeasureType, Proposition, Exp, AtomExp, ExpData, DataStruct, Expression

class Theorist:
    general: Knowledge
    specific: Dict[str, SpecificModel]

    def __init__(self):
        self.general = Knowledge.default()
        experiment_list = self.general.fetch_exps()
        self.specific = {}
        for name in experiment_list:
            self.specific[name] = SpecificModel(name, self.general.fetch_expstruct(name))

    def theoretical_analysis(self, exp_name: str):
        assert(exp_name in self.specific)
        spm: SpecificModel = self.specific[exp_name]
        data_info = spm.pick_relevant_exprs()
        res: List[Tuple[Exp, ExpData]] = search_relations(data_info)
        for (expr, expdata) in res:
            name: str = None
            if expdata.is_zero():
                name = spm.append_zero_exp(expr)
            elif expdata.is_conserved():
                name = spm.append_conserved_exp(expr)
            else:
                raise ValueError("search_relations(data_info) returned an unexpected result")
            if name is not None:
                expression: Expression = self.general.generalize(exp_name, str(expr))
                self.general.register_expr(str(expression))
                for key in self.specific:
                    self.specific[key].knowledge.register_expr(str(expression))
        pass

# 一个非常简餐粗暴的函数 （用于测试，详见 test8.py ）
# 将一个理论家记忆中的所有概念实例化 （specialize） 到一个实验中的具体表达式
# 再对具体表达式进行各种加减乘除求导的拼凑组合求值，
# 如果结果守恒，就将这个表达式注册为新的概念 （generalize） 
def work_at_exp(knowledge: Knowledge, exp_name: str) -> ExpStructure:
    exp = knowledge.fetch_expstruct(exp_name)
    exp.random_settings()
    exp.collect_expdata(MeasureType.default())
    for key in knowledge.fetch_concepts():
        specific_exprs: list[AtomExp] = knowledge.specialize_concept(key, exp_name)
        for i in specific_exprs:
            knowledge.eval(str(i), exp)
    data_info: DataStruct = exp.data_info()
    print(data_info)
    res: List[Tuple[Exp, ExpData]] = search_relations(data_info)
    for (expr, expdata) in res:
        if expdata.is_zero():
            prop = Proposition.IsZero(expr)
            knowledge.register_conclusion(str(prop))
        elif expdata.is_conserved():
            prop = Proposition.IsConserved(expr)
            knowledge.register_conclusion(str(prop))
        expression: Expression = knowledge.generalize(exp_name, str(expr))
        knowledge.register_expr(str(expression))
    return exp
