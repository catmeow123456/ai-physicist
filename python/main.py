from typing import Dict, List, Tuple, Set
from specific_model import SpecificModel
from object_model import ObjectModel
from interface import Knowledge
from interface import (
    search_relations, DataStruct, ExpStructure, MeasureType, Proposition,
    Exp, TExp, SExp, IExpConfig, ObjAttrExp, AtomExp, ExpData, DataStruct, Expression
)


def list_datainfo(data_info: DataStruct):
    df = data_info.fetch_atomexps();
    for i in df:
        print(i)

class Theorist:
    general: Knowledge
    specific: Dict[str, SpecificModel]
    objmodel: Dict[str, ObjectModel]

    def __init__(self):
        self.general = Knowledge.default()
        experiment_list = self.general.fetch_exps()
        self.specific = {}
        for name in experiment_list:
            self.specific[name] = SpecificModel(name, self.general.fetch_expstruct(name))
        self.objmodel = {}

    def newObjectModel(self, obj_type: str) -> ObjectModel:
        return ObjectModel(obj_type, self.general)

    def register_new_objattrexp(self, obj_type: str, objattrexp: ObjAttrExp):
        if not self.objmodel.__contains__(obj_type):
            self.objmodel[obj_type] = self.newObjectModel(obj_type)
        self.objmodel[obj_type].register_objattrexp(objattrexp)

    def theoretical_analysis(self, exp_name: str):
        assert(exp_name in self.specific)
        spm: SpecificModel = self.specific[exp_name]
        data_info: DataStruct = spm.pick_relevant_exprs()
        # list_datainfo(data_info)
        # for spe in specific_exprs:
        #     print(f"eval({spe} = ", self.knowledge.K.eval_exp_keyvaluehashed(spe).get_data())
        res: List[Tuple[Exp, ExpData]] = search_relations(data_info)
        print(f"Found {len(res)} relations")
        for (expr, expdata) in res:
            name: str = None
            if expdata.is_zero:
                name = spm.append_zero_exp(expr)
            elif expdata.is_conserved:
                name = spm.append_conserved_exp(expr)
                is_intrinsic, relevant_id = spm.check_intrinsic(expr)
                if relevant_id is not None:
                    relevant_id: List[int] = list(relevant_id)
                    if is_intrinsic and len(relevant_id) == 1:
                        print(f"Found intrinsic relation: {expr} with relevant_id = {relevant_id}")
                        id, obj_type = relevant_id[0], str(spm.experiment.get_obj_type(relevant_id[0]))
                        iexp_config = IExpConfig.Mk(
                            obj_type,
                            IExpConfig.From(exp_name),
                            id
                        )
                        objattrexp = ObjAttrExp.From(SExp.Mk(iexp_config, expr))
                        self.register_new_objattrexp(obj_type, objattrexp)
                    if is_intrinsic and len(relevant_id) == 2:
                        print(f"Found intrinsic relation: {expr} with relevant_id = {relevant_id}")
                        id, obj_type = relevant_id[0], str(spm.experiment.get_obj_type(relevant_id[0]))
                        iexp_config = IExpConfig.Mk(
                            obj_type,
                            IExpConfig.From(exp_name),
                            id
                        )
                        id1 = relevant_id[1]
                        standard_object_name = self.general.register_object(spm.experiment.get_obj(relevant_id[1]))
                        iexp_config = IExpConfig.Mkfix(
                            standard_object_name,
                            iexp_config,
                            id1
                        )
                        objattrexp = ObjAttrExp.From(SExp.Mk(iexp_config, expr))
                        self.register_new_objattrexp(obj_type, objattrexp)
            else:
                raise ValueError("search_relations(data_info) returned an unexpected result")
            if name is not None:
                expression: Expression = self.general.generalize(exp_name, expr)
                self.register_concept(expression.unwrap_texp())
        self.specific[exp_name].reduce_conclusions(debug=True)
        # for name, expr in self.specific[exp_name].conserved_list:
        #     expression: Expression = self.general.generalize(exp_name, expr)
        #     self.register_concept(expression.unwrap_texp())

    def register_concept(self, concept: TExp):
        expression: Expression = Expression.TExp(texp=concept)
        self.general.register_expr(expression)
        for key in self.specific:
            self.specific[key].knowledge.register_expr(expression)


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
        if expdata.is_zero:
            prop = Proposition.IsZero(expr)
            knowledge.register_conclusion(str(prop))
        elif expdata.is_conserved:
            prop = Proposition.IsConserved(expr)
            knowledge.register_conclusion(str(prop))
        expression: Expression = knowledge.generalize(exp_name, str(expr))
        knowledge.register_expr(str(expression))
    return exp
