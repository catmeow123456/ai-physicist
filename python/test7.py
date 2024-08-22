from interface import Knowledge
from ai_physicist import MeasureType
from ai_physicist import Expression, AtomExp
theorist = Knowledge.default()
print(theorist.fetch_exps())
oscil = theorist.fetch_expstruct("oscillation")
oscil.random_settings()
oscil.collect_expdata(MeasureType.default())
print(oscil.data_info())
theorist.eval("posr[2] - posx[1]", oscil).is_zero
texp: Expression = theorist.generalize("oscillation", "posr[2] - posx[1]")
print(texp)
texp: Expression = theorist.generalize("oscillation", "D[posx[1]'']/D[posx[1]]")
print(texp)
theorist.register_expr(str(texp), "concept")
assert(texp.unwrap_texp() == theorist.fetch_concept_texp("concept"))

print(theorist.fetch_concepts())
res: list[AtomExp] = theorist.specialize_concept("concept", "oscillation")
print(res[0].get_vec_ids())
print(texp.unwrap_texp().subst(res[0].get_vec_ids()))

res = theorist.specialize(str(texp), "oscillation")
print(res[0])