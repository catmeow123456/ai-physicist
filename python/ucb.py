import numpy as np
from typing import Tuple

class nsUCB:
    """
    non stationary Upper Confidence Bound algorithm
    """
    def __init__(self, info: str = None, alpha=0.1, cupt=1.0):
        self.alpha = alpha
        self.cupt = cupt
        self.value = 0
        self.info = info
        self.reset()

    def reset(self):
        self.count = 0
        self.value = 0

    def update(self, reward: float):
        if reward is None:
            self.count = self.count * (1 - self.alpha)
        else:
            self.value += (reward - self.value) * self.alpha
            self.count = self.count * (1 - self.alpha) + 1 * self.alpha

    def ucb(self):
        return self.value + self.cupt * np.sqrt(1.0 / (1.0 + self.count))

    def to_json(self) ->  Tuple[float, float, float, float, str | None]:
        return (self.alpha, self.cupt, self.value, self.count, self.info)

    def from_json(data: Tuple[float, float, float, float, str | None]) -> "nsUCB":
        obj = object.__new__(nsUCB)
        alpha, cupt, value, count, info = data
        obj.alpha = alpha
        obj.cupt = cupt
        obj.value = value
        obj.count = count
        obj.info = info
        return obj
