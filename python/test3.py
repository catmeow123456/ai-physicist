from ai_physicist import struct_oscillation
from ai_physicist import sentence, ObjAttrExp, MeasureType, Knowledge
exp_oscillation = struct_oscillation()
sexp = sentence.parse_sexp("#oscillation(1->MassPoint)|-D[posx[1]'']/D[posx[1]]")
print(str(sexp))
objattr = ObjAttrExp.From(sexp)
print(sentence.parse_str("#oscillation|-D[posx[1]'']/D[posx[1]]"))
KK = Knowledge()
exp_oscillation.random_settings()
expdata = exp_oscillation.collect_expdata(MeasureType.default())
exp = sentence.parse_exp("D[posx[1]'']/D[posx[1]]")
result = KK.eval(exp0=exp, context=exp_oscillation)
print(result.is_conserved(), result.is_zero())
# print(result.data)