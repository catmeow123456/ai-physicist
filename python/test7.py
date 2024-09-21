from interface import Knowledge
from ai_physicist import MeasureType
from ai_physicist import Expression, AtomExp
theorist = Knowledge.default()
print(theorist.fetch_exps)
oscil = theorist.fetch_expstruct("oscillation")
oscil.random_settings()
oscil.collect_expdata(MeasureType.default())
print(oscil.data_info)
theorist.eval("posr[2] - posx[1]", oscil).is_zero
concept: Expression = theorist.generalize("oscillation", "posr[2] - posx[1]")
print(concept)
concept: Expression = theorist.generalize("oscillation", "D[posx[1]'']/D[posx[1]]")
print(concept)
theorist.register_expr(str(concept), "concept")
assert(concept.unwrap_concept == theorist.fetch_concept_by_name("concept"))

print(theorist.fetch_concepts)
res: list[AtomExp] = theorist.specialize_concept("concept", "oscillation")
print(res[0].vec_ids)
print(concept.unwrap_concept.subst(res[0].vec_ids))

res = theorist.specialize(str(concept), "oscillation")
print(res[0])