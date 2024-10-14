from typing import Dict, Any, List
from ucb import nsUCB
from interface import AtomExp, Exp, Knowledge

def dict_to_json(d: Dict[str, Any]) -> Dict[str, str]:
    return {k: str(v) for k, v in d.items()}

class Bandit:
    """
    非平稳多臂老虎机
    """
    actions: Dict[str, nsUCB]

    def __init__(self):
        self.actions = {}

    def to_json(self):
        return {key: value.to_json() for key, value in self.actions.items()}

    def from_json(data: Dict[str, Any]) -> "Bandit":
        obj = object.__new__(Bandit)
        obj.actions = {key: nsUCB.from_json(value) for key, value in data.items()}
        return obj

    def register_action(self, name: str, info: str = None):
        """
        新注册一个动作
        """
        self.actions[name] = nsUCB(info)
        self.actions[name].update(1.0)

    def choose_actions(self, num: int) -> List[str]:
        """
        以 ub 值为排序，选择最优的 num 个动作
        """
        v = [(i, self.actions[i].ucb()) for i in self.actions]
        v.sort(key=lambda x: x[1], reverse=True)
        return [i[0] for i in v[:num]]

    def update_rewards(self, rewards: Dict[str, float]):
        """
        选定的动作获得了回报，更新每个动作的 nsUCB。
        """
        for key, item in self.actions.items():
            item.update(rewards.get(key, None))

class Memory:
    """
    AI 的记忆仓库，目的是在传入具体实验名称 exp_name 时，
    回想起一些相关的原子表达式，并根据权重抽取原子表达式 （pick_relevant_exprs）。
    抽取算法是基于非平稳多臂老虎机的，
    每一个具体实验都对应一个老虎机 specific[exp_name]，
    一个老虎机有一个动作空间，每一个动作都对应一个原子表达式。
    不同实验的经验是可以相互迁移的，因此有一个通用的老虎机 general，
    它的动作空间是所有实验的并集。如果在实验 A 中概念 v 有很好的表现，那么在其他实验中会有更高的倾向去选择 v。
    """
    knowledge: Knowledge
    general: Bandit
    specific: Dict[str, Bandit]

    def __init__(self, knowledge: Knowledge):
        self.knowledge = knowledge
        self.general = Bandit()
        self.specific = {}

    def to_json(self) -> Dict[str, Any]:
        return {
            "general": self.general.to_json(),
            "specific": {key: value.to_json() for key, value in self.specific.items()}
        }

    def from_json(data: Dict[str, Any], knowledge: Knowledge) -> "Memory":
        obj = object.__new__(Memory)
        obj.knowledge = knowledge
        obj.general = Bandit.from_json(data["general"])
        obj.specific = {key: Bandit.from_json(value) for key, value in data["specific"].items()}
        return obj

    def pick_relevant_exprs(self, exp_name: str) -> List[AtomExp]:
        """
        这个函数的目的是选取当前实验中的一些 specific 的原子表达式 （ 例如 posx[1], v[2] 等等 ） 。
        这些 specific 的原子表达式由概念库中的概念 specialize 生成，以备后续组合出更复杂的表达式。
        TODO：需要有方向性的智能的随机选取，且这种随机选取方式是可学习的
        """
        actions = self.specific[exp_name].choose_actions(6)
        print("Pick out Concepts: ", actions)
        exprs = []
        for action in actions:
            info = self.specific[exp_name].actions[action].info
            if info is not None:
                list_exps: list[Exp] = self.knowledge.specialize(concept=info, exp_name=exp_name)
                specific_exprs: list[AtomExp] = [i.unwrap_atom for i in list_exps]
            else:
                specific_exprs: list[AtomExp] = self.knowledge.specialize_concept(concept_name=action, exp_name=exp_name)
            exprs.extend(specific_exprs)
        return exprs

    def register_action(self, action_name: str):
        self.general.register_action(action_name)
        for exp_name in self.specific:
            self.specific[exp_name].register_action(action_name)

    def update_rewards(self, exp_name: str, rewards: Dict[str, float]):
        self.general.update_rewards(rewards)
        self.specific[exp_name].update_rewards(rewards)
