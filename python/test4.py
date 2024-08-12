from ai_physicist import struct_oscillation
from ai_physicist import sentence, ObjAttrExp, Objstructure, Knowledge

sexp = sentence.parse_sexp("#oscillation (1 -> MassPoint) (2 -> Spring) |- D[posx[1]'']/D[posx[1]]")
objattr = ObjAttrExp.From(sexp)
print(str(sexp))
mp = Objstructure.make_masspoint(1, 3)
mp.random_settings()
sp = Objstructure.make_spring(1.8, 2.2, 4, 6)
sp.random_settings()
newexp = struct_oscillation()
newexp.random_settings()

KK = Knowledge()
expdata1 = KK.eval_objattr(objattr, [mp, sp])
print(expdata1.is_conserved())

KK2 = Knowledge()
expdata2 = KK2.eval_objattr(objattr, [mp, sp])
print(expdata2.is_conserved())

print((expdata1 - expdata2).is_zero())