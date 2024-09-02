from main import Theorist

theorist = Theorist()
theorist.theoretical_analysis("motion0")
theorist.specific["motion0"].print_sympy_conclusion()
# 打印粗体字
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("motion")
theorist.specific["motion"].print_sympy_conclusion()

print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("oscillation", ver='trivial')
theorist.specific["oscillation"].print_sympy_conclusion()
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("oscillation")
theorist.specific["oscillation"].print_sympy_conclusion()

print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("collision", ver='trivial')
theorist.specific["collision"].print_sympy_conclusion()
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("collision", ver='ver2')
theorist.specific["collision"].print_sympy_conclusion()

""" Log: Wait for debug
Found 2 relations
Registered New Concept: C_01 = (1->Clock) (2->MassPoint) |- D[t[1]]/D[posx[2]]
Registered New Concept: C_02 = (1->MassPoint) (2->Clock) |- D[posx[1]]/D[t[2]]
R_02 conserved: D[posx[1]]/D[t[0]] = Derivative(posx_1(t_0), t_0)
--------------------
Found 3 relations
Registered New Concept: C_03 = (1->Clock) (2->MassPoint) |- D[t[1]]/D[C_02[2, 1]]
Registered New Concept: C_04 = (1->MassPoint) (2->Clock) |- D[C_02[1, 2]]/D[t[2]]
R_01 conserved: D[t[0]]/D[C_02[1, 0]] = 1/Derivative(posx_1(t_0), (t_0, 2))
--------------------
Found 11 relations
Registered New Concept: C_05 = (1->MassPoint) (2->Spring) |- (posx[1] / posr[2])
Registered New Concept: C_06 = (1->MassPoint) (2->Spring) |- D[posx[1]]/D[posr[2]]
Found intrinsic relation: D[posx[1]]/D[C_04[1, 0]] with relevant_id = [1, 2]
Registered New Onebody Intrinsic Concept: C_07 = [#oscillation (1->MassPoint) [2->Obj_01] |- D[posx[1]]/D[C_04[1, 0]]]
Registered New Concept: C_08 = (1->MassPoint) (2->Clock) |- D[posx[1]]/D[C_04[1, 2]]
Registered New Concept: C_09 = (1->Spring) (2->MassPoint) |- (posr[1] - posx[2])
Registered New Concept: C_10 = (1->Spring) (2->MassPoint) |- (posr[1] / posx[2])
Registered New Concept: C_11 = (1->Spring) (2->MassPoint) |- D[posr[1]]/D[posx[2]]
Found intrinsic relation: D[posr[2]]/D[C_04[1, 0]] with relevant_id = [1, 2]
Registered New Onebody Intrinsic Concept: C_12 = [#oscillation (1->MassPoint) [2->Obj_02] |- D[posr[2]]/D[C_04[1, 0]]]
Registered New Concept: C_13 = (1->Spring) (2->MassPoint) (3->Clock) |- D[posr[1]]/D[C_04[2, 3]]
Found intrinsic relation: D[C_04[1, 0]]/D[posx[1]] with relevant_id = [1, 2]
Registered New Concept: C_15 = (1->MassPoint) (2->Clock) |- D[C_04[1, 2]]/D[posx[1]]
Found intrinsic relation: D[C_04[1, 0]]/D[posr[2]] with relevant_id = [1, 2]
Registered New Concept: C_17 = (1->MassPoint) (2->Clock) (3->Spring) |- D[C_04[1, 2]]/D[posr[3]]
R_04 zero: (posr[2] - posx[1]) = posr_2(t_0) - posx_1(t_0)
R_03 conserved: D[posx[1]]/D[C_04[1, 0]] = Derivative(posx_1(t_0), t_0)/Derivative(posx_1(t_0), (t_0, 3))
--------------------
Found 16 relations
Registered New Concept: C_18 = (1->Clock) (2->MassPoint) |- D[t[1]]/D[(C_01[1, 2] * posx[2])]
Registered New Concept: C_20 = (1->MassPoint) (2->Clock) |- D[posx[1]]/D[(C_02[1, 2] * t[2])]
Registered New Concept: C_22 = (1->Clock) (2->MassPoint) |- D[(C_01[1, 2] * posx[2])]/D[t[1]]
Registered New Concept: C_23 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_01[1, 2] * posx[2])]/D[(posx[3] * C_01[1, 3])]
Registered New Concept: C_26 = (1->MassPoint) (2->Clock) |- D[(C_02[1, 2] * t[2])]/D[posx[1]]
Registered New Concept: C_27 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_02[1, 2] * posx[3])]/D[(C_02[3, 2] * posx[1])]
debug in maple--------------------
with (DifferentialAlgebra)
with (Tools)
R := DifferentialRing(blocks = [[R_11(),R_07(),R_06(),R_10(),R_09(),R_04(),R_02(),R_12(),R_01(),R_08(),R_03(),R_05(),posx_1(t_0),posx_2(t_0)]], derivations = [t_0])
ideal := [PretendRegularDifferentialChain([((t_0) * (diff(posx_1(t_0), t_0))) + ((-1) * (R_01()) * (posx_1(t_0))), ((t_0) * (diff(posx_2(t_0), t_0))) + ((-1) * (R_02()) * (posx_2(t_0))), (R_11()) + ((-1) * (R_01())), ((-1) * (R_02())) + ((R_01()) * (R_10())), (-1) + ((R_02()) * (R_04()))], R), PretendRegularDifferentialChain([diff(posx_1(t_0), t_0$2), diff(posx_2(t_0), t_0$2), (-1) + (R_11()), (-1) + (R_10()), (-1) + (R_04()), (-1) + (R_02()), (-1) + (R_01())], R)]
print(BelongsTo([(((diff(((diff(posx_1(t_0), t_0))^(-1)) * (posx_1(t_0)), t_0))^(2)) * ((diff(posx_1(t_0), t_0))^(3)) * (((-1) * ((diff(posx_2(t_0), t_0))^(2)) * (diff(posx_2(t_0), t_0$2))) + ((-1) * (((-2) * ((diff(posx_2(t_0), t_0$2))^(2))) + ((diff(posx_2(t_0), t_0)) * (diff(posx_2(t_0), t_0$3)))) * (posx_2(t_0))))) + (((diff(posx_2(t_0), t_0))^(3)) * ((((diff(posx_1(t_0), t_0))^(2)) * (diff(posx_1(t_0), t_0$2))) + ((((-2) * ((diff(posx_1(t_0), t_0$2))^(2))) + ((diff(posx_1(t_0), t_0)) * (diff(posx_1(t_0), t_0$3)))) * (posx_1(t_0)))) * (diff(((diff(posx_1(t_0), t_0))^(-1)) * (posx_1(t_0)), t_0)) * (diff(((diff(posx_2(t_0), t_0))^(-1)) * (posx_2(t_0)), t_0)))], ideal))
debug in maple--------------------
Traceback (most recent call last):
  File "/home/fyl/ai-physicist/python/test9.py", line 14, in <module>
    theorist.theoretical_analysis("collision", ver='ver2')
  File "/home/fyl/ai-physicist/python/main.py", line 97, in theoretical_analysis
    self.specific[exp_name].reduce_conclusions(debug=False)
  File "/home/fyl/ai-physicist/python/specific_model.py", line 124, in reduce_conclusions
    if ideal.belongs_to(new_eq):
  File "/home/fyl/ai-physicist/python/diffalg/diffalg.py", line 138, in belongs_to
    stdout = solver.exec_maple()
  File "/home/fyl/ai-physicist/python/diffalg/mapleIO.py", line 38, in exec_maple
    raise Exception(self.outputs[-1])
Exception: Error,(inDifferentialAlgebra:-BelongsTo)apolynomial,alistorasetofpolynomialsisexpectedasfirstparameter,received:[(-1/diff(posx_1(t_0),t_0)^2*posx_1(t_0)*diff(diff(posx_1(t_0),t_0),t_0)+1)^2*diff(posx_1(t_0),t_0)^3*(-diff(posx_2(t_0),t_0)^2*diff(diff(posx_2(t_0),t_0),t_0)-(-2*diff(diff(posx_2(t_0),t_0),t_0)^2+diff(posx_2(t_0),t_0)*diff(diff(diff(posx_2(t_0),t_0),t_0),t_0))*posx_2(t_0))+diff(posx_2(t_0),t_0)^3*(diff(posx_1(t_0)...
"""