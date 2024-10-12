from typing import Dict, Any, List
from ucb import nsUCB
from interface import (
    AtomExp, Exp, Knowledge,
    Proposition, Concept, Intrinsic, DataStruct, ExpStructure
)

def dict_to_json(d: Dict[str, Any]) -> Dict[str, str]:
    return {k: str(v) for k, v in d.items()}

class Memory:
    """
    AI 的记忆仓库，其中包括了各种概念或者其他抽象的表达，
    且每一个概念拥有一个权重（ TODO ）。
    给记忆仓库一个 pick_relevant_exprs 指令，并传入特定实验 experiment、Knowledge，
    它会根据记忆联想到与实验 experiment 相关的一些特定的原子表达式。
    后续会接入神经网络来调节这一部分。
    """
    actions: Dict[str, nsUCB]

    def __init__(self):
        self.actions = {}

    def to_json(self):
        return {key: value.to_json() for key, value in self.actions.items()}

    def from_json(data: Dict[str, Any]) -> "Memory":
        obj = object.__new__(Memory)
        obj.actions = {key: nsUCB.from_json(value) for key, value in data.items()}
        return obj

    def register_action(self, name: str, info: str = None):
        """
        新注册一个动作
        """
        self.actions[name] = nsUCB(info)

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
