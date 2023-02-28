import sys

def create_res(cons, res):
    if len(cons) <= 2:
        res += [str(x) for x in cons]
    else:
        res += [f"{cons[0]}-{cons[-1]}"]

numbers = sorted([int(x) for x in sys.stdin.read().split()][1:])

res = []
cons = [numbers[0]]

for i in range(1, len(numbers)):
    if numbers[i] == numbers[i-1]+1:
        cons.append(numbers[i])
    else:
        create_res(cons, res)
        cons = [numbers[i]]
    
    if i == len(numbers)-1:
        create_res(cons, res)

print(" ".join(res))


