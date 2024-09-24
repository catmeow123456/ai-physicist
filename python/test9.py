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
theorist.save_to_file("data/knowledge.txt", "data/memory.json")

theorist = Theorist.read_from_file("data/knowledge.txt", "data/memory.json")
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("collision", ver='trivial')
theorist.specific["collision"].print_sympy_conclusion()
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("collision", ver='ver2')
theorist.specific["collision"].print_sympy_conclusion()
theorist.save_to_file("data/knowledge2.txt", "data/memory2.json")
