from typing import List, Dict, Set
from deprecated.sphinx import deprecated

class AtomExp:
    """
    Represents an atomic expression in the language of the AI Physicist.
    For example, "pos[1]", "r[2, 3]", "t[0]" are all atomic expressions.
    """

    def __new__(cls, content: str) -> AtomExp: ...

    def __str__(self) -> str: ...

    def __hash__(self) -> int: ...

    def VariableIds(name: str, ids: List[int]) -> AtomExp:
        """
        Create an atomic expression with a variable name and a list of ids.
        For example AtomExp.VariableIds("x", [1, 2]) represents "x[1, 2]". 
        """
        ...

    def get_t() -> AtomExp:
        """
        Get t[0] as an AtomExp.
        In general, we default the time variable to t[0],
        to indicate that the number of the “clock” is 0.
        """
        ...

    def get_name(self) -> str: ...
    def get_vec_ids(self) -> List[int]:
        """get the ordered ids of the atomic expression"""
        ...
    def get_allids(self) -> Set[int]: ...
    def substs(self, subs: Dict[int, int]) -> AtomExp:
        """Substitute the ids of the atomic expression with the given substitutions."""
        ...

class Exp:
    """
    Represents an expression in the language of the AI Physicist.
    """
    ...
    def Number(i: int) -> Exp:
        """
        Create a number expression.
        """
        ...

# class Expression:
#     ...
#     def Exp