import random

for i in range(500):
	inp=""
	s=0.0
	for j in range(81):
		r = random.uniform(-1.0, 1.0)
		inp += str(r) + " "
		s += r
	out=""
	for j in range(81):
		out += str(random.uniform(0.0, 1.0)) + " "
	out += str(random.uniform(-1.0, 1.0))
	print(inp)
	print(out)
