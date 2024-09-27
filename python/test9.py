from main import Theorist

def step_one():
    """
    依次做匀速直线运动、匀变速直线运动、简谐振动的理论分析，从而发现质量概念
    将获得的知识库保存到 knowledge.txt 和 memory.json 
    """
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

# step_one()

theorist = Theorist.read_from_file("data/knowledge.txt", "data/memory.json")
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("collision", ver='trivial')
theorist.specific["collision"].print_sympy_conclusion()
print('\033[1m'+('-'*20)+'\033[0m')
theorist.theoretical_analysis("collision", ver='ver2')
theorist.specific["collision"].print_sympy_conclusion()
theorist.save_to_file("data/knowledge2.txt", "data/memory2.json")
