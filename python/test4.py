from ai_physicist import struct_oscillation
from ai_physicist import sentence, ObjAttrExp, Objstructure, Knowledge

sexp = sentence.parse_sexp("#oscillation (1 -> MassPoint) (2 -> Spring) |- D[posx[1]'']/D[posx[1]]")
objattr = ObjAttrExp.From(sexp)
print(str(sexp))
mp = Objstructure.make_masspoint(2, 5)
mp.random_settings()
sp = Objstructure.make_spring(1.8, 2.2, 4, 6)
sp.random_settings()
print(str(mp))
print(str(sp))
newexp = struct_oscillation()
newexp.random_settings()

KK = Knowledge.default()
expdata1 = KK.eval_objattr(objattr, [mp, sp])
print(expdata1)

KK2 = Knowledge.default()
expdata2 = KK2.eval_objattr(objattr, [mp, sp])
print(expdata2)

print(expdata1 - expdata2)