from main import Theorist

theorist = Theorist()
theorist.theoretical_analysis("motion0")
theorist.specific["motion0"].print_sympy_conclusion()
print('-'*20)
theorist.theoretical_analysis("motion")
theorist.specific["motion"].print_sympy_conclusion()
print('-'*20)
theorist.theoretical_analysis("oscillation")
theorist.specific["oscillation"].print_sympy_conclusion()
print('-'*20)
theorist.theoretical_analysis("collision")
theorist.specific["collision"].print_sympy_conclusion()