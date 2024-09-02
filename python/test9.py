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
Found intrinsic relation: D[C_03[1, 0]]/D[posr[2]] with relevant_id = [1, 2]
Registered New Onebody Intrinsic Concept: C_06 = [#oscillation (1->MassPoint) [2->Obj_01] |- D[C_03[1, 0]]/D[posr[2]]]
Registered New Concept: C_06 = [#oscillation (1->MassPoint) [2->Obj_01] |- D[C_03[1, 0]]/D[posr[2]]]
Registered New Concept: C_07 = (1->MassPoint) (2->Clock) (3->Spring) |- D[C_03[1, 2]]/D[posr[3]]
Found intrinsic relation: D[C_03[1, 0]]/D[posx[1]] with relevant_id = [1, 2]
Registered New Onebody Intrinsic Concept: C_08 = [#oscillation (1->MassPoint) [2->Obj_02] |- D[C_03[1, 0]]/D[posx[1]]]
Registered New Concept: C_08 = [#oscillation (1->MassPoint) [2->Obj_02] |- D[C_03[1, 0]]/D[posx[1]]]
Registered New Concept: C_09 = (1->MassPoint) (2->Clock) |- D[C_03[1, 2]]/D[posx[1]]
Found intrinsic relation: D[posr[2]]/D[C_03[1, 0]] with relevant_id = [1, 2]
Registered New Concept: C_11 = (1->Spring) (2->MassPoint) (3->Clock) |- D[posr[1]]/D[C_03[2, 3]]
Registered New Concept: C_12 = (1->Spring) (2->MassPoint) |- (posr[1] / posx[2])
Registered New Concept: C_13 = (1->Spring) (2->MassPoint) |- D[posr[1]]/D[posx[2]]
Found intrinsic relation: D[posx[1]]/D[C_03[1, 0]] with relevant_id = [1, 2]
Registered New Concept: C_15 = (1->MassPoint) (2->Clock) |- D[posx[1]]/D[C_03[1, 2]]
Registered New Concept: C_16 = (1->MassPoint) (2->Spring) |- (posx[1] - posr[2])
Registered New Concept: C_17 = (1->MassPoint) (2->Spring) |- (posx[1] / posr[2])
Registered New Concept: C_18 = (1->MassPoint) (2->Spring) |- D[posx[1]]/D[posr[2]]
P8 zero: (posx[1] - posr[2]) = -posr_2(t_0) + posx_1(t_0)
P1 conserved: posl[2] = posl_2(t_0)
P2 conserved: D[C_03[1, 0]]/D[posr[2]] = Derivative(posx_1(t_0), (t_0, 3))/Derivative(posr_2(t_0), t_0)
--------------------
exp_list = [VariableIds { name: "C_06", ids: [1] }, VariableIds { name: "C_06", ids: [2] }]
exp_list = [VariableIds { name: "C_08", ids: [1] }, VariableIds { name: "C_08", ids: [2] }]
Found 13 relations
Registered New Concept: C_20 = (1->Clock) (2->MassPoint) |- D[C_02[1, 2]]/D[t[1]]
Found intrinsic relation: C_08[1] with relevant_id = [1]
Found intrinsic relation: C_08[2] with relevant_id = [2]
Found intrinsic relation: C_06[1] with relevant_id = [1]
P1 zero: D[C_01[2, 0]]/D[t[0]] = Derivative(posx_2(t_0), (t_0, 2))
P4 zero: D[C_02[0, 1]]/D[t[0]] = Derivative(1/Derivative(posx_1(t_0), t_0), t_0)
--------------------
exp_list = [VariableIds { name: "C_06", ids: [1] }, VariableIds { name: "C_06", ids: [2] }]
exp_list = [VariableIds { name: "C_08", ids: [1] }, VariableIds { name: "C_08", ids: [2] }]
Found 336 relations
Registered New Concept: C_28 = (1->MassPoint) (2->Clock) |- D[posx[1]]/D[(t[2] * C_01[1, 2])]
Found intrinsic relation: (posx[2] / (C_08[1] * posx[2])) with relevant_id = [1]
Found intrinsic relation: (posx[2] / (C_08[2] * posx[2])) with relevant_id = [2]
Found intrinsic relation: D[posx[2]]/D[(C_08[2] * posx[2])] with relevant_id = [2]
Found intrinsic relation: (posx[2] / (C_06[1] * posx[2])) with relevant_id = [1]
Found intrinsic relation: (C_01[2, 0] / (C_08[1] * C_01[2, 0])) with relevant_id = [1]
Found intrinsic relation: (C_01[2, 0] / (C_08[2] * C_01[2, 0])) with relevant_id = [2]
Found intrinsic relation: (C_01[2, 0] / (C_06[1] * C_01[2, 0])) with relevant_id = [1]
Registered New Concept: C_36 = (1->MassPoint) (2->Clock) (3->MassPoint) |- (C_01[1, 2] / (C_06[1] * C_02[2, 3]))
Registered New Concept: C_37 = (1->Clock) (2->MassPoint) |- D[t[1]]/D[(C_02[1, 2] * posx[2])]
Found intrinsic relation: (t[0] / (C_08[1] * t[0])) with relevant_id = [1]
Found intrinsic relation: (t[0] / (C_08[2] * t[0])) with relevant_id = [2]
Found intrinsic relation: (t[0] / (C_06[1] * t[0])) with relevant_id = [1]
Found intrinsic relation: (C_02[0, 2] / (C_08[1] * C_02[0, 2])) with relevant_id = [1]
Found intrinsic relation: (C_02[0, 2] / (C_08[2] * C_02[0, 2])) with relevant_id = [2]
Found intrinsic relation: (C_02[0, 2] / (C_06[1] * C_02[0, 2])) with relevant_id = [1]
Registered New Concept: C_45 = (1->Clock) (2->MassPoint) (3->MassPoint) |- (C_02[1, 2] / (C_06[2] * C_01[3, 1]))
Found intrinsic relation: (posx[1] / (C_08[1] * posx[1])) with relevant_id = [1]
Found intrinsic relation: (posx[1] / (C_08[2] * posx[1])) with relevant_id = [2]
Found intrinsic relation: D[posx[1]]/D[(C_08[2] * posx[1])] with relevant_id = [2]
Found intrinsic relation: (posx[1] / (C_06[1] * posx[1])) with relevant_id = [1]
Found intrinsic relation: (C_02[0, 1] / (C_02[0, 1] * C_08[1])) with relevant_id = [1]
Found intrinsic relation: (C_02[0, 1] / (C_08[2] * C_02[0, 1])) with relevant_id = [2]
Found intrinsic relation: (C_02[0, 1] / (C_06[1] * C_02[0, 1])) with relevant_id = [1]
Found intrinsic relation: (C_01[1, 0] / (C_01[1, 0] * C_08[1])) with relevant_id = [1]
Found intrinsic relation: (C_01[1, 0] / (C_01[1, 0] * C_08[2])) with relevant_id = [2]
Found intrinsic relation: (C_01[1, 0] / (C_06[1] * C_01[1, 0])) with relevant_id = [1]
Registered New Concept: C_57 = (1->Clock) (2->MassPoint) |- D[(t[1] * C_01[2, 1])]/D[posx[2]]
Registered New Concept: C_58 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(t[1] * C_01[2, 1])]/D[(C_08[3] * posx[2])]
Found intrinsic relation: D[(t[0] * C_01[2, 0])]/D[(C_08[2] * posx[2])] with relevant_id = [2]
Registered New Onebody Intrinsic Concept: C_59 = [#collision (2->MassPoint) |- D[(t[0] * C_01[2, 0])]/D[(C_08[2] * posx[2])]]
Registered New Concept: C_59 = [#collision (2->MassPoint) |- D[(t[0] * C_01[2, 0])]/D[(C_08[2] * posx[2])]]
Registered New Concept: C_60 = (1->Clock) (2->MassPoint) |- D[(t[1] * C_01[2, 1])]/D[(C_08[2] * posx[2])]
Registered New Concept: C_61 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(t[1] * C_01[2, 1])]/D[(C_06[3] * posx[2])]
Registered New Concept: C_62 = (1->Clock) (2->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[t[1]]
Registered New Concept: C_63 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(C_08[3] * t[1])]
Registered New Concept: C_64 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(C_02[1, 3] * posx[3])]
Registered New Concept: C_65 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(C_06[3] * t[1])]
Registered New Concept: C_66 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_02[1, 2] * t[1])]/D[(C_08[2] * posx[3])]
Registered New Concept: C_67 = (1->MassPoint) (2->MassPoint) (3->Clock) |- D[(posx[1] * C_01[2, 3])]/D[(C_01[1, 3] * posx[2])]
Registered New Concept: C_68 = (1->MassPoint) (2->MassPoint) (3->Clock) |- D[(posx[1] * C_01[2, 3])]/D[(C_06[1] * t[3])]
Found intrinsic relation: ((C_08[1] * posx[2]) / posx[2]) with relevant_id = [1]
Registered New Concept: C_70 = (1->MassPoint) (2->MassPoint) (3->Clock) |- D[(C_08[1] * posx[2])]/D[(t[3] * C_01[2, 3])]
Found intrinsic relation: ((C_08[1] * posx[2]) / (C_08[2] * posx[2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[1] * C_01[2, 0]) / C_01[2, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_08[1] * C_01[2, 0]) * C_02[0, 2]) with relevant_id = [1]
Found intrinsic relation: ((C_08[1] * C_01[2, 0]) / (C_08[2] * C_01[2, 0])) with relevant_id = [1, 2]
Registered New Concept: C_75 = (1->MassPoint) (2->MassPoint) (3->Clock) |- ((C_08[1] * C_01[2, 3]) / (C_06[2] * C_02[3, 1]))
Found intrinsic relation: ((C_08[1] * t[0]) / t[0]) with relevant_id = [1]
Registered New Concept: C_77 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_08[1] * t[2])]/D[(C_02[2, 3] * posx[3])]
Registered New Concept: C_78 = (1->MassPoint) (2->Clock) |- D[(C_08[1] * t[2])]/D[(C_02[2, 1] * posx[1])]
Found intrinsic relation: ((C_08[1] * t[0]) / (C_08[2] * t[0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[1] * C_02[0, 2]) * C_01[2, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_08[1] * C_02[0, 2]) / C_02[0, 2]) with relevant_id = [1]
Found intrinsic relation: ((C_08[1] * C_02[0, 2]) * (C_08[1] * C_01[2, 0])) with relevant_id = [1]
Found intrinsic relation: ((C_08[1] * C_02[0, 2]) / (C_08[2] * C_02[0, 2])) with relevant_id = [1, 2]
Registered New Concept: C_84 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_08[1] * C_02[2, 3]) / (C_06[3] * C_01[1, 2]))
Found intrinsic relation: ((C_08[1] * posx[1]) / posx[1]) with relevant_id = [1]
Found intrinsic relation: ((C_08[1] * posx[1]) / (C_08[2] * posx[1])) with relevant_id = [1, 2]
Registered New Concept: C_87 = (1->MassPoint) (2->Clock) |- D[(C_08[1] * posx[1])]/D[(C_01[1, 2] * t[2])]
Registered New Concept: C_88 = (1->Clock) (2->MassPoint) (3->MassPoint) |- D[(C_02[1, 2] * t[1])]/D[(C_08[3] * posx[3])]
Registered New Concept: C_91 = (1->Clock) (2->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(C_08[2] * t[1])]
Registered New Concept: C_92 = (1->Clock) (2->MassPoint) |- D[(C_02[1, 2] * posx[2])]/D[(C_06[2] * t[1])]
Found intrinsic relation: ((C_02[0, 1] * C_08[1]) / C_02[0, 1]) with relevant_id = [1]
Found intrinsic relation: ((C_02[0, 1] * C_08[1]) * C_01[1, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_02[0, 1] * C_08[1]) / (C_08[2] * C_02[0, 1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * posx[2]) / posx[2]) with relevant_id = [2]
Found intrinsic relation: D[(C_08[2] * posx[2])]/D[(t[0] * C_01[2, 0])] with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * posx[2]) / (C_08[1] * posx[2])) with relevant_id = [1, 2]
Registered New Concept: C_100 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_08[1] * posx[1])]/D[(C_02[2, 3] * t[2])]
Found intrinsic relation: ((C_08[2] * posx[2]) / (C_06[1] * posx[2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) / C_01[2, 0]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) * C_02[0, 2]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) / (C_08[1] * C_01[2, 0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) * (C_08[1] * C_02[0, 2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_01[2, 0]) / (C_06[1] * C_01[2, 0])) with relevant_id = [1, 2]
Registered New Concept: C_107 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_08[1] * C_01[1, 2]) / (C_06[1] * C_02[2, 3]))
Found intrinsic relation: ((C_08[2] * t[0]) / t[0]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * t[0]) / (C_08[1] * t[0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * t[0]) / (C_06[1] * t[0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) * C_01[2, 0]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) / C_02[0, 2]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) * (C_08[1] * C_01[2, 0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) / (C_08[1] * C_02[0, 2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) * (C_08[2] * C_01[2, 0])) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 2]) / (C_06[1] * C_02[0, 2])) with relevant_id = [1, 2]
Registered New Concept: C_117 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_08[1] * C_02[2, 1]) / (C_06[1] * C_01[3, 2]))
Found intrinsic relation: ((C_08[2] * posx[1]) / posx[1]) with relevant_id = [2]
Found intrinsic relation: D[(C_08[2] * posx[1])]/D[posx[1]] with relevant_id = [2]
Registered New Concept: C_120 = (1->MassPoint) (2->MassPoint) (3->Clock) |- D[(C_08[1] * posx[2])]/D[(C_02[3, 1] * t[3])]
Found intrinsic relation: ((C_08[2] * posx[1]) / (C_08[1] * posx[1])) with relevant_id = [1, 2]
Found intrinsic relation: D[(C_08[2] * posx[1])]/D[(C_01[1, 0] * t[0])] with relevant_id = [2]
Registered New Onebody Intrinsic Concept: C_122 = [#collision (2->MassPoint) |- D[(C_08[2] * posx[1])]/D[(C_01[1, 0] * t[0])]]
Registered New Concept: C_122 = [#collision (2->MassPoint) |- D[(C_08[2] * posx[1])]/D[(C_01[1, 0] * t[0])]]
Found intrinsic relation: ((C_08[2] * posx[1]) / (C_06[1] * posx[1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_02[0, 1]) / C_02[0, 1]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 1]) * C_01[1, 0]) with relevant_id = [2]
Found intrinsic relation: ((C_08[2] * C_02[0, 1]) / (C_02[0, 1] * C_08[1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_08[2] * C_02[0, 1]) / (C_06[1] * C_02[0, 1])) with relevant_id = [1, 2]
Registered New Concept: C_130 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_01[1, 2] * posx[3])]/D[(C_06[1] * t[2])]
Found intrinsic relation: D[(C_01[1, 0] * t[0])]/D[(C_08[2] * posx[1])] with relevant_id = [2]
Registered New Concept: C_135 = (1->MassPoint) (2->Clock) |- D[(C_01[1, 2] * t[2])]/D[(C_06[1] * posx[1])]
Found intrinsic relation: ((C_01[1, 0] * C_08[1]) * C_02[0, 1]) with relevant_id = [1]
Found intrinsic relation: ((C_01[1, 0] * C_08[1]) / C_01[1, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_01[1, 0] * C_08[1]) * (C_02[0, 1] * C_08[1])) with relevant_id = [1]
Found intrinsic relation: ((C_01[1, 0] * C_08[1]) * (C_08[2] * C_02[0, 1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_01[1, 0] * C_08[1]) / (C_01[1, 0] * C_08[2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) * C_02[0, 1]) with relevant_id = [2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) / C_01[1, 0]) with relevant_id = [2]
Registered New Concept: C_143 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_01[1, 2] * C_08[3]) + (C_08[1] * C_01[3, 2]))
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) * (C_02[0, 1] * C_08[1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) * (C_08[2] * C_02[0, 1])) with relevant_id = [2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) / (C_01[1, 0] * C_08[1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_01[1, 0] * C_08[2]) / (C_06[1] * C_01[1, 0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_06[1] * posx[2]) / posx[2]) with relevant_id = [1]
Registered New Concept: C_149 = (1->MassPoint) (2->MassPoint) (3->Clock) |- D[(C_06[1] * posx[2])]/D[(t[3] * C_01[2, 3])]
Registered New Concept: C_150 = (1->MassPoint) (2->MassPoint) |- ((C_06[1] * posx[2]) - (C_08[1] * posx[2]))
Found intrinsic relation: ((C_06[1] * posx[2]) / (C_08[2] * posx[2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_06[1] * C_01[2, 0]) / C_01[2, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_01[2, 0]) * C_02[0, 2]) with relevant_id = [1]
Registered New Concept: C_154 = (1->MassPoint) (2->MassPoint) (3->Clock) |- ((C_06[1] * C_01[2, 3]) - (C_08[1] * C_01[2, 3]))
Found intrinsic relation: ((C_06[1] * C_01[2, 0]) * (C_08[1] * C_02[0, 2])) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_01[2, 0]) / (C_08[2] * C_01[2, 0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_06[1] * C_01[2, 0]) * (C_08[2] * C_02[0, 2])) with relevant_id = [1, 2]
Registered New Concept: C_158 = (1->MassPoint) (2->MassPoint) (3->Clock) |- ((C_06[1] * C_01[2, 3]) + (C_01[1, 3] * C_08[2]))
Registered New Concept: C_159 = (1->MassPoint) (2->MassPoint) (3->Clock) |- ((C_06[1] * C_01[2, 3]) / (C_06[2] * C_02[3, 1]))
Found intrinsic relation: ((C_06[1] * t[0]) / t[0]) with relevant_id = [1]
Registered New Concept: C_161 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_06[1] * t[2])]/D[(C_02[2, 3] * posx[3])]
Registered New Concept: C_162 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_06[1] * t[2])]/D[(posx[1] * C_01[3, 2])]
Registered New Concept: C_163 = (1->MassPoint) (2->Clock) |- ((C_06[1] * t[2]) - (C_08[1] * t[2]))
Registered New Concept: C_164 = (1->MassPoint) (2->Clock) |- D[(C_06[1] * t[2])]/D[(C_02[2, 1] * posx[1])]
Found intrinsic relation: ((C_06[1] * t[0]) / (C_08[2] * t[0])) with relevant_id = [1, 2]
Registered New Concept: C_166 = (1->MassPoint) (2->Clock) (3->MassPoint) |- D[(C_06[1] * t[2])]/D[(C_01[1, 2] * posx[3])]
Found intrinsic relation: ((C_06[1] * C_02[0, 2]) * C_01[2, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_02[0, 2]) / C_02[0, 2]) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_02[0, 2]) * (C_08[1] * C_01[2, 0])) with relevant_id = [1]
Registered New Concept: C_170 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 3]) - (C_08[1] * C_02[2, 3]))
Found intrinsic relation: ((C_06[1] * C_02[0, 2]) * (C_08[2] * C_01[2, 0])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_06[1] * C_02[0, 2]) / (C_08[2] * C_02[0, 2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_06[1] * C_02[0, 2]) * (C_06[1] * C_01[2, 0])) with relevant_id = [1]
Registered New Concept: C_174 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 3]) / (C_06[3] * C_01[1, 2]))
Found intrinsic relation: ((C_06[1] * posx[1]) / posx[1]) with relevant_id = [1]
Registered New Concept: C_176 = (1->MassPoint) |- ((C_06[1] * posx[1]) - (C_08[1] * posx[1]))
Found intrinsic relation: ((C_06[1] * posx[1]) / (C_08[2] * posx[1])) with relevant_id = [1, 2]
Registered New Concept: C_178 = (1->MassPoint) (2->Clock) |- D[(C_06[1] * posx[1])]/D[(C_01[1, 2] * t[2])]
Found intrinsic relation: ((C_06[1] * C_02[0, 1]) / C_02[0, 1]) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_02[0, 1]) * C_01[1, 0]) with relevant_id = [1]
Registered New Concept: C_181 = (1->MassPoint) (2->Clock) |- ((C_06[1] * C_02[2, 1]) - (C_02[2, 1] * C_08[1]))
Found intrinsic relation: ((C_06[1] * C_02[0, 1]) / (C_08[2] * C_02[0, 1])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_06[1] * C_02[0, 1]) * (C_01[1, 0] * C_08[1])) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_02[0, 1]) * (C_01[1, 0] * C_08[2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_06[1] * C_01[1, 0]) * C_02[0, 1]) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_01[1, 0]) / C_01[1, 0]) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_01[1, 0]) * (C_02[0, 1] * C_08[1])) with relevant_id = [1]
Found intrinsic relation: ((C_06[1] * C_01[1, 0]) * (C_08[2] * C_02[0, 1])) with relevant_id = [1, 2]
Registered New Concept: C_189 = (1->MassPoint) (2->Clock) |- ((C_06[1] * C_01[1, 2]) - (C_01[1, 2] * C_08[1]))
Found intrinsic relation: ((C_06[1] * C_01[1, 0]) / (C_01[1, 0] * C_08[2])) with relevant_id = [1, 2]
Found intrinsic relation: ((C_06[1] * C_01[1, 0]) * (C_06[1] * C_02[0, 1])) with relevant_id = [1]
Registered New Concept: C_193 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_01[1, 2]) / C_02[2, 3])
Registered New Concept: C_194 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_01[1, 2]) / (C_02[2, 3] * C_08[3]))
Registered New Concept: C_196 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_01[1, 2]) / (C_08[1] * C_02[2, 3]))
Registered New Concept: C_197 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_01[1, 2]) * (C_01[3, 2] * C_08[3]))
Registered New Concept: C_198 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_01[1, 2]) * (C_01[3, 2] * C_08[1]))
Registered New Concept: C_200 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_01[1, 2]) * (C_06[3] * C_01[3, 2]))
Registered New Concept: C_201 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_01[1, 2]) / (C_06[1] * C_02[2, 3]))
Registered New Concept: C_203 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) * C_02[2, 3])
Registered New Concept: C_204 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) + (C_02[2, 3] * C_08[3]))
Registered New Concept: C_205 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) * (C_02[2, 3] * C_08[3]))
Registered New Concept: C_207 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) * (C_08[1] * C_02[2, 3]))
Registered New Concept: C_208 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) / (C_01[3, 2] * C_08[3]))
Registered New Concept: C_209 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) / (C_01[3, 2] * C_08[1]))
Registered New Concept: C_210 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) + (C_06[3] * C_02[2, 3]))
Registered New Concept: C_211 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) * (C_06[3] * C_02[2, 3]))
Registered New Concept: C_213 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 1]) / (C_06[1] * C_01[3, 2]))
Registered New Concept: C_216 = (1->MassPoint) (2->Clock) (3->MassPoint) |- ((C_06[1] * C_02[2, 3]) * (C_06[1] * C_02[2, 1]))
Registered New Concept: C_219 = (1->MassPoint) (2->MassPoint) (3->Clock) |- ((C_06[1] * C_01[2, 3]) + (C_06[2] * C_01[1, 3]))
Registered New Concept: C_220 = (1->MassPoint) (2->MassPoint) (3->Clock) |- ((C_06[1] * C_01[2, 3]) * (C_06[1] * C_01[1, 3]))
P1 zero: D[C_01[2, 0]]/D[t[0]] = Derivative(posx_2(t_0), (t_0, 2))
P4 zero: D[C_02[0, 1]]/D[t[0]] = Derivative(1/Derivative(posx_1(t_0), t_0), t_0)
P49 zero: ((C_06[1] * posx[2]) - (C_08[1] * posx[2])) = C_06_1*posx_2(t_0) - C_08_1*posx_2(t_0)
P55 zero: ((C_06[1] * t[0]) - (C_08[1] * t[0])) = C_06_1*t_0 - C_08_1*t_0
P64 zero: ((C_06[2] * posx[2]) - (C_08[2] * posx[2])) = C_06_2*posx_2(t_0) - C_08_2*posx_2(t_0)
P74 zero: ((C_06[2] * t[0]) - (C_08[2] * t[0])) = C_06_2*t_0 - C_08_2*t_0
"""