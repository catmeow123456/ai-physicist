from main import Theorist

def step_init():
    theorist = Theorist()
    theorist.save_to_file("data/init")

def step_onebyone():
    theorist: Theorist = Theorist.read_from_file("data/init")
    exp_list = ["motion0", "motion", "stringmotion0", "oscillation", "collision"]
    for i in range(10):
        for exp in exp_list:
            theorist.theoretical_analysis(exp, ver='trivial')
            theorist.theoretical_analysis(exp, ver='ver2')
            theorist.theoretical_analysis(exp, ver='ver3')
            theorist.specific[exp].print_sympy_conclusion()
            theorist.save_to_file("data/num" + str(i))


step_init()
step_onebyone()
