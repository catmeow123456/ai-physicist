from ai_physicist import sentence, Exp
print(sentence.parse_str("(x[1]' + x[2]') ** 2"))
a = sentence.parse_exp("(x[1]' + x[2]') ** 2")
t = sentence.parse_exp("t[0]")
b = Exp.DiffExp(a, t, 1)
print(str(b))