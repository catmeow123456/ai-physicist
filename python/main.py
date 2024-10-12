import json
from typing import Dict, List, Tuple, Set
from memory import Memory
from specific_model import SpecificModel, ConservedInfo
from object_model import ObjectModel
from interface import Knowledge
from interface import (
    search_relations_ver2, search_trivial_relations, search_relations_ver3,
    search_relations, DataStruct, ExpStructure, MeasureType, Proposition,
    Exp, Concept, SExp, IExpConfig, Intrinsic, AtomExp, ExpData, DataStruct, Expression
)
from tqdm import tqdm


def list_datainfo(data_info: DataStruct):
    df = data_info.data_keys
    for i in df:
        print(i)

class Theorist:
    """
    Theorist 类，由一个主要 Knowledge 类 `general` 
    和一系列特殊 Model 类 `specific`,`objmodel` 组成。
    general 是一个全局的知识库。
    specific 代表了关于每一个实验的知识库（也被叫做“具体模型”）。
    objmodel 代表了关于每一个物体的知识库（刻画了对物体的认识）。
    理论家的主要工作就是不断地做实验，然后从实验中发现一些规律，将这些规律注册到 general 或者具体的 specific 中，
    如果这个过程中发现一些关于物理对象的具体知识，这个知识会被注册到 objmodel 中。
    理论家可以调用不同知识库的记忆，来辅助推理分析，也可以将推理分析的结果注册到不同的知识库中。
    """
    general: Knowledge
    memory: Memory
    specific: Dict[str, SpecificModel]
    objmodel: Dict[str, ObjectModel]

    def __init__(self):
        self.general = Knowledge.default()
        self.memory = Memory()
        experiment_list = self.general.fetch_exps
        self.specific = {}
        for name in experiment_list:
            self.specific[name] = SpecificModel(name, self.general)
        self.objmodel = {}

    def read_from_file(filename_for_knowledge: str, filename_for_memory: str) -> "Theorist":
        obj = object.__new__(Theorist)
        obj.general = Knowledge.read_from_file(filename_for_knowledge)
        with open(filename_for_memory, "r") as f:
            memory_dict = json.load(f)
        obj.memory = Memory.from_json(memory_dict["general"])
        obj.specific = {}
        for name in obj.general.fetch_exps:
            obj.specific[name] = SpecificModel(name, obj.general)
            obj.specific[name].load_json(memory_dict["specific"][name])
        obj.objmodel = {}
        return obj

    def save_to_file(self, filename_for_knowledge: str, filename_for_memory: str):
        self.general.save_to_file(filename_for_knowledge)
        memory_dict = {
            "general": self.memory.to_json(),
            "specific": {
                key: value.to_json() for key, value in self.specific.items()
            }
        }
        with open(filename_for_memory, "w") as f:
            json.dump(memory_dict, f, indent=4)

    def newObjectModel(self, obj_type: str) -> ObjectModel:
        return ObjectModel(obj_type, self.general)

    def register_new_intrinsic(self, obj_type: str, intrinsic: Intrinsic) -> str:
        if not self.objmodel.__contains__(obj_type):
            self.objmodel[obj_type] = self.newObjectModel(obj_type)
        name = self.objmodel[obj_type].register_intrinsic(intrinsic)
        if name is not None:
            print("\033[1m" + f"Registered New Concept: {name} = {intrinsic}" + "\033[0m")
            for key in self.specific:
                self.specific[key].memory.register_action(name)
        return name

    def theoretical_analysis(self, exp_name: str, ver: str | None = None):
        assert (exp_name in self.specific)
        spm: SpecificModel = self.specific[exp_name]
        data_info: DataStruct = spm.pick_relevant_exprs()
        conclusion_before = set(spm.conclusions.keys())
        # list_datainfo(data_info)
        if ver is None:
            res: List[Tuple[Exp, ExpData]] = search_relations(data_info)
        elif ver == 'ver2':
            res: List[Tuple[Exp, ExpData]] = search_relations_ver2(data_info)
        elif ver == 'ver3':
            res: List[Tuple[Exp, ExpData]] = search_relations_ver3(data_info)
        elif ver == 'trivial':
            res: List[Tuple[Exp, ExpData]] = search_trivial_relations(data_info)
        print(f"Found {len(res)} relations")
        for (expr, expdata) in tqdm(res):
            name: str = None
            if expdata.is_zero:
                name = spm.append_zero_exp(expr)
            elif expdata.is_conserved:
                name = spm.append_conserved_exp(expr)
            else:
                raise ValueError("search_relations(data_info) returned an unexpected result")
        # 去除冗余关系
        print(f"Reducing {len(spm.conclusions.keys())} conclusions")
        spm.reduce_conclusions(debug=False)
        # 注册概念
        rewards = {}
        conclusion_after = set(spm.conclusions.keys())
        conclusion_diff = conclusion_after - conclusion_before
        for name in conclusion_diff:
            expr: Exp = spm.conclusions.get(name).unwrap_exp
            expression: Expression = self.general.generalize(exp_name, expr)
            self.register_concept(expression.unwrap_concept)
            actions: Set[str] = {i.name for i in expr.all_atoms}
            for action in actions: # 枚举表达式的原子，计算 action reward
                rewards[action] = rewards.get(action, 0) + 1 / len(actions)
        # 将 intrinsic_buffer 中的内禀概念注册到知识库中
        self.register_intrinsics(spm.intrinsic_buffer)
        for key in spm.intrinsic_buffer:
            cqinfo = spm.intrinsic_buffer[key]
            actions: Set[str] = {i.name for i in cqinfo.exp.all_atoms}
            for action in actions:
                rewards[action] = rewards.get(action, 0) + 1 / len(actions)
        spm.intrinsic_buffer.clear()
        # update reward to spm.memory
        spm.memory.update_rewards(rewards)

    def register_intrinsics(self, CQinfos: Dict[str, ConservedInfo]):
        for name, info in CQinfos.items():
            assert info.is_intrinsic and info.relevant_id is not None
            exp_name = info.exp_name
            experiment = self.specific[exp_name].experiment
            relevant_id = list(info.relevant_id)
            expr = info.exp
            if len(relevant_id) == 1:
                print(f"Found intrinsic relation: {expr} with relevant_id = {relevant_id}")
                id, obj_type = relevant_id[0], str(experiment.get_obj_type(relevant_id[0]))
                iexp_config = IExpConfig.Mk(
                    obj_type,
                    IExpConfig.From(exp_name),
                    id
                )
                intrinsic = Intrinsic.From(SExp.Mk(iexp_config, expr))
                self.register_new_intrinsic(obj_type, intrinsic)
            if len(relevant_id) == 2:
                print(f"Found intrinsic relation: {expr} with relevant_id = {relevant_id}")

                id, obj_type = relevant_id[1], str(experiment.get_obj_type(relevant_id[1]))
                iexp_config = IExpConfig.Mk(
                    obj_type, IExpConfig.From(exp_name), id
                )
                id1, obj_type1 = relevant_id[0], str(experiment.get_obj_type(relevant_id[0]))
                standard_object_name = self.general.register_object(experiment.get_obj(relevant_id[0]))
                iexp_config = IExpConfig.Mkfix(
                    standard_object_name, iexp_config, id1
                )
                intrinsic = Intrinsic.From(SExp.Mk(iexp_config, expr))
                name = self.register_new_intrinsic(obj_type, intrinsic)
                if name is None:
                    continue
                new_exp = Exp.Atom(AtomExp.VariableIds(name, [id])) / expr
                new_info = self.specific[exp_name].make_conserved_info(None, new_exp)
                if new_info.is_intrinsic and new_info.relevant_id == {id1}:
                    new_iexp_config = IExpConfig.Mk(
                        obj_type1, IExpConfig.From(exp_name), id1
                    )
                    intrinsic = Intrinsic.From(SExp.Mk(new_iexp_config, new_exp))
                    self.register_new_intrinsic(obj_type1, intrinsic)

    def register_concept(self, concept: Concept):
        """
        Theorist 类中新注册一个概念。

        新注册的概念在 general 中注册过后，可以有选择性地给 specific 中的每一个实验注册
        这个地方有很多优化空间，因为在一些实验中某个概念可能是毫无用处的，这个时候就可以删掉。
        """
        expression: Expression = Expression.Concept(concept=concept)
        name = self.general.register_expr(expression)
        if name is not None:
            tqdm.write(f"\033[1m" + f"Registered New Concept: {name} = {concept}" + f"\033[0m")
            for key in self.specific:
                self.specific[key].memory.register_action(name)


def work_at_exp(knowledge: Knowledge, exp_name: str) -> ExpStructure:
    """
    一个非常简单粗暴的函数 （用于测试，详见 test8.py ）
    将一个理论家记忆中的所有概念实例化 （specialize） 到一个实验中的具体表达式
    再对具体表达式进行各种加减乘除求导的拼凑组合求值，
    如果结果守恒，就将这个表达式注册为新的概念 （generalize）
    """
    exp = knowledge.fetch_expstruct(exp_name)
    exp.random_settings()
    exp.collect_expdata(MeasureType.default())
    for key in knowledge.fetch_concepts:
        specific_exprs: list[AtomExp] = knowledge.specialize_concept(key, exp_name)
        for i in specific_exprs:
            knowledge.eval(str(i), exp)
    data_info: DataStruct = exp.data_info
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
