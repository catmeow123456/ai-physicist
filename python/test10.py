from main import Theorist

def step_one():
    theorist = Theorist()

    theorist.theoretical_analysis("motion0", ver='trivial')
    theorist.specific["motion0"].print_sympy_conclusion()
    theorist.theoretical_analysis("motion0")
    theorist.specific["motion0"].print_sympy_conclusion()
    theorist.theoretical_analysis("motion", ver='trivial')
    theorist.specific["motion"].print_sympy_conclusion()
    theorist.theoretical_analysis("motion")
    theorist.specific["motion"].print_sympy_conclusion()

    theorist.theoretical_analysis("stringmotion0", ver='trivial')
    theorist.specific["stringmotion0"].print_sympy_conclusion()
    theorist.theoretical_analysis("stringmotion0")
    theorist.specific["stringmotion0"].print_sympy_conclusion()

    theorist.theoretical_analysis("oscillation", ver='trivial')
    theorist.specific["oscillation"].print_sympy_conclusion()
    theorist.theoretical_analysis("oscillation")
    theorist.specific["oscillation"].print_sympy_conclusion()
    theorist.save_to_file("data/a_knowledge.txt", "data/a_memory.json")

def step_two():
    theorist = Theorist.read_from_file("data/a_knowledge.txt", "data/a_memory.json")
    theorist.theoretical_analysis("oscillation", ver='ver3')    

step_one()
step_two()
