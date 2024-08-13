from interface import Theorist
from ai_physicist import MeasureType
from ai_physicist import search_relations
theorist = Theorist()
print(theorist.fetch_exps())
oscil = theorist.fetch_expstruct("oscillation")
oscil.random_settings()
oscil.collect_expdata(MeasureType.default())
print(str(oscil.data_info()))
theorist.eval("posr[2] - posx[1]", oscil).is_zero()
texp = theorist.generalize("oscillation", "posr[2] - posx[1]")
print(str(texp))
texp = theorist.generalize("oscillation", "D[posx[1]'']/D[posx[1]]")
print(str(texp))
theorist.register_expr("concept", str(texp))
print(theorist.fetch_concepts())
res = theorist.K.specialize_concept("concept", "oscillation")
print(str(res[0]))
print(texp.unwrap_texp().subst([0, 1]))