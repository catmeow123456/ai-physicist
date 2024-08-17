#%%
from main import work_at_exp, Knowledge

knowledge = Knowledge.default()
#%%
print("Round 0")
exp = work_at_exp(knowledge, "motion0")
knowledge.print_concepts()
print("Round 1")
exp = work_at_exp(knowledge, "motion0")
knowledge.print_concepts()
print("Round 2")
exp = work_at_exp(knowledge, "motion0")
knowledge.print_concepts()
#%%
print("Round 3")
exp = work_at_exp(knowledge, "motion")
knowledge.print_concepts()
#%%
print("Round 4")
exp = work_at_exp(knowledge, "oscillation")
knowledge.print_concepts()
print("Round 5")
exp = work_at_exp(knowledge, "oscillation")
knowledge.print_concepts()
#%% print conclusions
knowledge.print_conclusions()