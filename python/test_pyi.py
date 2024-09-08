#%%
from ai_physicist import AtomExp
print(AtomExp.VariableIds("dis", [2, 3]))
print(AtomExp("dis[2,3]").get_allids())
print(AtomExp.get_t())
print(AtomExp.get_t().get_name())
dis23: AtomExp = AtomExp.VariableIds("dis", [2, 3])
print(dis23.get_name())
print(dis23.get_vec_ids())
print(dis23.substs({2: 3, 3: 33}))
#%%
from ai_physicist import Exp, AtomExp
print(Exp.Number(1))
exp_of_atom: Exp = Exp.Atom(AtomExp("dis[2,3]"))
print(exp_of_atom)
print(exp_of_atom.unwrap_atom().__class__)
print(exp_of_atom.complexity)

# %%
from ai_physicist import ExpData, NormalData
xx: ExpData = ExpData([[1, 1.1], [1.05, 0.9]])
print(xx.is_conserved)
print(xx)
print(ExpData([[1.1, 1.2, 1.3], [-3.1, 1.2, 1.4]]).is_zero)
x: ExpData = ExpData([[1.1, 1.2, 1.3, 1.2, 2.2, 2.1], [-3.1, 1.2, 1.4, 1.1, 3.3, 0.0]])
x = (-x * x - x)*x / x
print(x.is_conserved)
xn : NormalData = x.normal_data
print(xn.badpts)
# %%
from ai_physicist import ExpData, ConstData
x: ExpData = ExpData([[0.99999911, 1.0001, 1.0000001], [1.000001, 0.99990001, 1.00001]])
xc : ConstData = x.const_data
print(xc)
print(x.__powi__(2)/x.__powi__(2))
# %%
