from main import Theorist

theorist = Theorist()

theorist.theoretical_analysis("motion0", ver='trivial')
theorist.specific["motion0"].print_sympy_conclusion()
theorist.theoretical_analysis("motion0")
theorist.specific["motion0"].print_sympy_conclusion()
# 打印粗体字
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("motion", ver='trivial')
theorist.specific["motion"].print_sympy_conclusion()
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

"""
Found 2 relations
Registered New Concept: C_01 = (1->MassPoint) (2->Clock) |- D[posx[1]]/D[t[2]]
P1 conserved: D[posx[1]]/D[t[0]] = Derivative(posx_1(t_0), t_0)
Found 2 relations
Registered New Concept: C_02 = (1->Clock) (2->MassPoint) |- D[t[1]]/D[posx[2]]
P1 conserved: D[posx[1]]/D[t[0]] = Derivative(posx_1(t_0), t_0)
--------------------
Found 2 relations
Registered New Concept: C_03 = (1->MassPoint) (2->Clock) |- D[C_01[1, 2]]/D[t[2]]
P1 conserved: D[C_01[1, 0]]/D[t[0]] = Derivative(posx_1(t_0), (t_0, 2))
--------------------
Found 3 relations
Registered New Concept: C_04 = (1->Clock) (2->MassPoint) |- D[t[1]]/D[C_01[2, 1]]
P1 conserved: D[C_01[1, 0]]/D[t[0]] = Derivative(posx_1(t_0), (t_0, 2))
--------------------
Found 2 relations
P1 conserved: posl[2] = posl_2(t_0)
--------------------
Found 11 relations
Registered New Concept: C_06 = (1->MassPoint) (2->Spring) |- (posx[1] / posr[2])
Registered New Concept: C_07 = (1->MassPoint) (2->Spring) |- D[posx[1]]/D[posr[2]]
Found intrinsic relation: D[posx[1]]/D[C_03[1, 0]] with relevant_id = [1, 2]
Registered New Onebody Intrinsic Concept: C_08 = [#oscillation (1->MassPoint) [2->Obj_01] |- D[posx[1]]/D[C_03[1, 0]]]
Registered New Concept: C_08 = [#oscillation (1->MassPoint) [2->Obj_01] |- D[posx[1]]/D[C_03[1, 0]]]
Registered New Concept: C_09 = (1->MassPoint) (2->Clock) |- D[posx[1]]/D[C_03[1, 2]]
Registered New Concept: C_10 = (1->Spring) (2->MassPoint) |- (posr[1] - posx[2])
Registered New Concept: C_11 = (1->Spring) (2->MassPoint) |- (posr[1] / posx[2])
Registered New Concept: C_12 = (1->Spring) (2->MassPoint) |- D[posr[1]]/D[posx[2]]
Found intrinsic relation: D[posr[2]]/D[C_03[1, 0]] with relevant_id = [1, 2]
Registered New Onebody Intrinsic Concept: C_13 = [#oscillation (1->MassPoint) [2->Obj_02] |- D[posr[2]]/D[C_03[1, 0]]]
Registered New Concept: C_13 = [#oscillation (1->MassPoint) [2->Obj_02] |- D[posr[2]]/D[C_03[1, 0]]]
Registered New Concept: C_14 = (1->Spring) (2->MassPoint) (3->Clock) |- D[posr[1]]/D[C_03[2, 3]]
Found intrinsic relation: D[C_03[1, 0]]/D[posx[1]] with relevant_id = [1, 2]
Registered New Concept: C_16 = (1->MassPoint) (2->Clock) |- D[C_03[1, 2]]/D[posx[1]]
Found intrinsic relation: D[C_03[1, 0]]/D[posr[2]] with relevant_id = [1, 2]
Registered New Concept: C_18 = (1->MassPoint) (2->Clock) (3->Spring) |- D[C_03[1, 2]]/D[posr[3]]
P5 zero: (posr[2] - posx[1]) = posr_2(t_0) - posx_1(t_0)
P1 conserved: posl[2] = posl_2(t_0)
P4 conserved: D[posx[1]]/D[C_03[1, 0]] = Derivative(posx_1(t_0), t_0)/Derivative(posx_1(t_0), (t_0, 3))
--------------------
Found 13 relations
Registered New Concept: C_20 = (1->Clock) (2->MassPoint) |- D[C_02[1, 2]]/D[t[1]]
Found intrinsic relation: C_08[2] with relevant_id = [2]
Found intrinsic relation: C_13[1] with relevant_id = [1]
P1 zero: C_03[1, 0] = Derivative(posx_1(t_0), (t_0, 2))
P2 zero: D[C_02[0, 2]]/D[t[0]] = Derivative(1/Derivative(posx_2(t_0), t_0), t_0)
--------------------
Found 282 relations
Registered New Concept: C_27 = (1->Clock) (2->MassPoint) |- D[t[1]]/D[(C_02[1, 2] * posx[2])]
Found intrinsic relation: (t[0] / (C_08[2] * t[0])) with relevant_id = [2]
Found intrinsic relation: (t[0] / (C_13[1] * t[0])) with relevant_id = [1]
Registered New Concept: C_31 = (1->MassPoint) (2->Clock) |- D[posx[1]]/D[(C_01[1, 2] * t[2])]
Found intrinsic relation: (posx[2] / (C_08[2] * posx[2])) with relevant_id = [2]
Found intrinsic relation: (posx[2] / (C_13[1] * posx[2])) with relevant_id = [1]
Found intrinsic relation: (C_02[0, 2] / (C_08[2] * C_02[0, 2])) with relevant_id = [2]
Found intrinsic relation: (C_02[0, 2] / (C_13[1] * C_02[0, 2])) with relevant_id = [1]
Found intrinsic relation: (C_02[0, 1] / (C_08[2] * C_02[0, 1])) with relevant_id = [2]
Found intrinsic relation: (C_02[0, 1] / (C_13[1] * C_02[0, 1])) with relevant_id = [1]
Found intrinsic relation: (C_01[2, 0] / (C_08[2] * C_01[2, 0])) with relevant_id = [2]
Found intrinsic relation: (C_01[2, 0] / (C_13[1] * C_01[2, 0])) with relevant_id = [1]
Found intrinsic relation: (C_01[1, 0] / (C_01[1, 0] * C_08[2])) with relevant_id = [2]
Found intrinsic relation: (C_01[1, 0] / (C_01[1, 0] * C_13[1])) with relevant_id = [1]
Found intrinsic relation: (posx[1] / (posx[1] * C_08[2])) with relevant_id = [2]
Found intrinsic relation: (posx[1] / (posx[1] * C_13[1])) with relevant_id = [1]
Registered New Concept: C_45 = (1->Clock) (2->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[t[1]]
Registered New Concept: C_46 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(C_13[3] * t[1])]
Registered New Concept: C_47 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(posx[3] * C_02[1, 3])]
Registered New Concept: C_48 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(C_08[3] * t[1])]
Registered New Concept: C_49 = (1->Clock) (2->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(C_13[2] * t[1])]
Registered New Concept: C_50 = (1->MassPoint) (2->Clock) |- D[(C_01[1, 2] * t[2])]/D[posx[1]]
Registered New Concept: C_51 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_01[1, 2] * t[2])]/D[(C_13[3] * posx[1])]
Registered New Concept: C_52 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_01[1, 2] * t[2])]/D[(C_08[3] * posx[1])]
Registered New Concept: C_53 = (1->MassPoint) (2->Clock) |- D[(C_01[1, 2] * t[2])]/D[(C_13[1] * posx[1])]
Found intrinsic relation: ((C_08[2] * t[0]) / t[0]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * t[0]) / (C_13[1] * t[0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * posx[2]) / posx[2]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * posx[2]) / (C_13[1] * posx[2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) / C_02[0, 2]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) * C_01[2, 0]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) / (C_13[1] * C_02[0, 2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_02[0, 1]) / C_02[0, 1]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 1]) * C_01[1, 0]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 1]) / (C_13[1] * C_02[0, 1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) * C_02[0, 2]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) / C_01[2, 0]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) * (C_08[2] * C_02[0, 2])) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) / (C_13[1] * C_01[2, 0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_13[1] * t[0]) / t[0]) with relevant_id = [1]
Registered New Concept: C_69 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_13[1] * t[2])]/D[(C_02[2, 3] * posx[3])]
Found intrinsic relation: ((C_13[1] * t[0]) / (C_08[2] * t[0])) with relevant_id = [1, 2]
Registered New Concept: C_71 = (1->MassPoint) (2->Clock) |- D[(C_13[1] * t[2])]/D[(posx[1] * C_02[2, 1])]
Found intrinsic relation: ((C_13[1] * posx[2]) / posx[2]) with relevant_id = [1]
Registered New Concept: C_73 = (1->MassPoint) (2->MassPoint) (3->Clock) |- D[(C_13[1] * posx[2])]/D[(C_01[2, 3] * t[3])]
Found intrinsic relation: ((C_13[1] * posx[2]) / (C_08[2] * posx[2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_13[1] * C_02[0, 2]) / C_02[0, 2]) with relevant_id = [1]
Found intrinsic relation: ((C_13[1] * C_02[0, 2]) * C_01[2, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_13[1] * C_02[0, 2]) / (C_08[2] * C_02[0, 2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_13[1] * C_02[0, 2]) * (C_08[2] * C_01[2, 0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_13[1] * C_02[0, 1]) / C_02[0, 1]) with relevant_id = [1]
Found intrinsic relation: ((C_13[1] * C_02[0, 1]) * C_01[1, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_13[1] * C_02[0, 1]) / (C_08[2] * C_02[0, 1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_13[1] * C_01[2, 0]) * C_02[0, 2]) with relevant_id = [1]
Found intrinsic relation: ((C_13[1] * C_01[2, 0]) / C_01[2, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_13[1] * C_01[2, 0]) * (C_08[2] * C_02[0, 2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_13[1] * C_01[2, 0]) / (C_08[2] * C_01[2, 0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_13[1] * C_01[2, 0]) * (C_13[1] * C_02[0, 2])) with relevant_id = [1]
Registered New Concept: C_89 = (1->MassPoint) (2->Clock) |- D[(C_01[1, 2] * t[2])]/D[(C_08[1] * posx[1])]
Registered New Concept: C_91 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_01[1, 2] * posx[3])]/D[(posx[1] * C_01[3, 2])]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) * C_02[0, 1]) with relevant_id = [2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) / C_01[1, 0]) with relevant_id = [2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) * (C_08[2] * C_02[0, 1])) with relevant_id = [2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) * (C_13[1] * C_02[0, 1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) / (C_01[1, 0] * C_13[1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_01[1, 0] * C_13[1]) * C_02[0, 1]) with relevant_id = [1]
Found intrinsic relation: ((C_01[1, 0] * C_13[1]) / C_01[1, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_01[1, 0] * C_13[1]) * (C_08[2] * C_02[0, 1])) with relevant_id = [1, 2]
Registered New Concept: C_100 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_01[1, 2] * C_13[1]) + (C_08[3] * C_01[3, 2]))
Found intrinsic relation: ((C_01[1, 0] * C_13[1]) * (C_13[1] * C_02[0, 1])) with relevant_id = [1]
Found intrinsic relation: ((C_01[1, 0] * C_13[1]) / (C_01[1, 0] * C_08[2])) with relevant_id = [1, 2]
Registered New Concept: C_106 = (1->MassPoint) (2->Clock) |- D[(posx[1] * C_02[2, 1])]/D[(C_08[1] * t[2])]
Found intrinsic relation: ((posx[1] * C_08[2]) / posx[1]) with relevant_id = [2]
Found intrinsic relation: ((posx[1] * C_08[2]) / (posx[1] * C_13[1])) with relevant_id = [1, 2]
Found intrinsic relation: ((posx[1] * C_13[1]) / posx[1]) with relevant_id = [1]
Registered New Concept: C_112 = (1->MassPoint) (2->Clock) |- D[(posx[1] * C_13[1])]/D[(C_01[1, 2] * t[2])]
Found intrinsic relation: ((posx[1] * C_13[1]) / (posx[1] * C_08[2])) with relevant_id = [1, 2]
Registered New Concept: C_114 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_08[1] * t[2])]/D[(C_02[2, 3] * posx[3])]
Registered New Concept: C_115 = (1->MassPoint) (2->Clock) |- ((C_08[1] * t[2]) - (C_13[1] * t[2]))
Registered New Concept: C_116 = (1->MassPoint) (2->Clock) |- D[(C_08[1] * t[2])]/D[(posx[1] * C_02[2, 1])]
Registered New Concept: C_117 = (1->MassPoint) (2->MassPoint) (3->Clock) |- D[(C_08[1] * posx[2])]/D[(C_01[2, 3] * t[3])]
Registered New Concept: C_118 = (1->MassPoint) (2->MassPoint) |- ((C_08[1] * posx[2]) - (C_13[1] * posx[2]))
Registered New Concept: C_119 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_08[1] * C_02[2, 3]) - (C_13[1] * C_02[2, 3]))
Registered New Concept: C_120 = (1->MassPoint) (2->Clock) |- ((C_08[1] * C_02[2, 1]) - (C_13[1] * C_02[2, 1]))
Registered New Concept: C_121 = (1->MassPoint) (2->MassPoint) (3->Clock) |- ((C_08[1] * C_01[2, 3]) - (C_13[1] * C_01[2, 3]))
Registered New Concept: C_122 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_08[1] * C_01[1, 2]) + (C_08[3] * C_01[3, 2]))
Registered New Concept: C_123 = (1->MassPoint) (2->Clock) |- ((C_08[1] * C_01[1, 2]) - (C_01[1, 2] * C_13[1]))
Registered New Concept: C_124 = (1->MassPoint) (2->Clock) |- D[(C_08[1] * posx[1])]/D[(C_01[1, 2] * t[2])]
Registered New Concept: C_125 = (1->MassPoint) |- ((C_08[1] * posx[1]) - (posx[1] * C_13[1]))
Registered New Concept: C_127 = (1->MassPoint) (2->Clock) |- ((C_13[1] * t[2]) - (C_08[1] * t[2]))
Registered New Concept: C_130 = (1->MassPoint) |- ((C_13[1] * posx[1]) - (C_08[1] * posx[1]))
Registered New Concept: C_131 = (1->MassPoint) (2->Clock) |- ((C_13[1] * C_02[2, 1]) - (C_08[1] * C_02[2, 1]))
Registered New Concept: C_132 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_13[1] * C_02[2, 3]) - (C_08[1] * C_02[2, 3]))
Registered New Concept: C_133 = (1->MassPoint) (2->Clock) |- ((C_13[1] * C_01[1, 2]) - (C_08[1] * C_01[1, 2]))
Registered New Concept: C_134 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_13[1] * C_01[1, 2]) + (C_01[3, 2] * C_13[3]))
Registered New Concept: C_136 = (1->MassPoint) (2->MassPoint) (3->Clock) |- ((C_13[1] * C_01[2, 3]) - (C_01[2, 3] * C_08[1]))
Registered New Concept: C_138 = (1->MassPoint) (2->MassPoint) |- ((C_13[1] * posx[2]) - (posx[2] * C_08[1]))
((C_01[1, 0] * C_13[1]) + (C_08[2] * C_01[2, 0])) --> C_08_2*Derivative(posx_2(t_0), t_0) + C_13_1*Derivative(posx_1(t_0), t_0)  --eq_reduced-->  C_08_2*Derivative(posx_2(t_0), t_0) + C_13_1*Derivative(posx_1(t_0), t_0)
P1 zero: C_03[1, 0] = Derivative(posx_1(t_0), (t_0, 2))
P2 zero: D[C_02[0, 2]]/D[t[0]] = Derivative(1/Derivative(posx_2(t_0), t_0), t_0)
P37 zero: ((C_08[1] * t[0]) - (C_13[1] * t[0])) = C_08_1*t_0 - C_13_1*t_0
P49 zero: ((C_13[2] * t[0]) - (C_08[2] * t[0])) = -C_08_2*t_0 + C_13_2*t_0
P28 conserved: ((C_01[1, 0] * C_13[1]) + (C_08[2] * C_01[2, 0])) = C_08_2*Derivative(posx_2(t_0), t_0) + C_13_1*Derivative(posx_1(t_0), t_0)
"""