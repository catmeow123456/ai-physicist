#%%
from main import work_at_exp, Theorist

theorist = Theorist()
#%%
print("Round 1")
exp = work_at_exp(theorist, "motion0")
theorist.print_concepts()
print("Round 2")
exp = work_at_exp(theorist, "motion0")
theorist.print_concepts()
#%%
print("Round 3")
exp = work_at_exp(theorist, "motion")
theorist.print_concepts()
#%%
print("Round 4")
exp = work_at_exp(theorist, "oscillation")
theorist.print_concepts()
