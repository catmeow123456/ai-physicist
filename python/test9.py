from main import Theorist

theorist = Theorist()
theorist.theoretical_analysis("motion0")
theorist.specific["motion0"].print_sympy_conclusion()
# 打印粗体字
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("motion")
theorist.specific["motion"].print_sympy_conclusion()
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("oscillation")
theorist.specific["oscillation"].print_sympy_conclusion()
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("collision")
theorist.specific["collision"].print_sympy_conclusion()