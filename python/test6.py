from interface import Theorist
from ai_physicist import MeasureType
from ai_physicist import search_relations
from ai_physicist import sentence

theorist = Theorist()
theorist.fetch_exps()
s = theorist.fetch_expstruct("motion0")
s.random_settings()
ds = s.collect_expdata(MeasureType.default())
print(str(s.data_info()))
print(s.data_info().fetch_data("posx", 1).__diff__(
    s.data_info().fetch_data("t", 0)).is_conserved())
res = search_relations(s.data_info())
for i in res:
    print(i[0])
theorist.register_expr("MP1",
                       "(1->MassPoint)|-posx[1]'")
print(theorist.fetch_concepts()['MP1'])
s = theorist.fetch_expstruct("motion")
s.random_settings()
s.collect_expdata(MeasureType.default())
theorist.eval("MP1[1]", s)
print(str(s.data_info()))
res = search_relations(s.data_info()) 
print(res)

texp = sentence.parse("(1->MassPoint)(2->MassPoint)|-posx[1]-posx[2]")
exp = texp.unwrap_texp().subst([2,1])
print(str(exp))
print(theorist.generalize("collision", str(exp)))
print(res[0][0])