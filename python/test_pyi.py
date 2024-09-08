#%%
from ai_physicist import Exp, AtomExp
print(Exp.Number(1))
print(AtomExp.VariableIds("dis", [2, 3]))
print(AtomExp.get_t())
print(AtomExp.get_t().get_name())
a: AtomExp = AtomExp.VariableIds("dis", [2, 3])
print(a.get_name())
print(a.get_vec_ids())
print(a.substs({2: 3, 3: 33}))
#%%