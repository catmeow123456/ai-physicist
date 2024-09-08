#%%
from ai_physicist import Exp, AtomExp
print(Exp.Number(1))
print(AtomExp.VariableIds("dis", [2, 3]))
print(AtomExp("dis[2,3]").get_allids())
print(AtomExp.get_t())
print(AtomExp.get_t().get_name())
dis23: AtomExp = AtomExp.VariableIds("dis", [2, 3])
print(dis23.get_name())
print(dis23.get_vec_ids())
print(dis23.substs({2: 3, 3: 33}))
#%%