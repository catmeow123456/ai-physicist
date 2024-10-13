import numpy as np
from typing import Tuple

cpuct = 1.0
alpha = 0.1             # 以一个较小的时间周期 decay
alpha_count = 0.001     # count 代表执行操作的次数，以一个较大的时间周期 decay

class nsUCB:
    """
    non stationary Upper Confidence Bound algorithm
    """
    def __init__(self, info: str = None):
        self.count = 0
        self.value = 0
        self.info = info

    def reset(self):
        self.count = 0
        self.value = 0

    def update(self, reward: float):
        if reward is None:
            self.count = self.count * (1 - alpha_count)
        else:
            self.value = self.value * (1 - alpha) + reward
            self.count = self.count * (1 - alpha_count) + 1

    def ucb(self):
        return self.value + cpuct * np.sqrt(1.0 / (1.0 + self.count))

    def to_json(self) ->  Tuple[float, float, str | None]:
        return (self.value, self.count, self.info)

    def from_json(data: Tuple[float, float, str | None]) -> "nsUCB":
        obj = object.__new__(nsUCB)
        value, count, info = data
        obj.value = value
        obj.count = count
        obj.info = info
        return obj
