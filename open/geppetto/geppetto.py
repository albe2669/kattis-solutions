import sys

[n, m] = [int(x) for x in sys.stdin.readline().split()]

pairs = set()

for i in range(m):
    [a, b] = [int(x) for x in sys.stdin.readline().split()]
    pairs.add((min(a, b), max(a, b)))

def combinations(ls, ingredient):
    if ingredient > n:
        return 1

    amount = combinations(ls, ingredient + 1)

    for other in ls:
        if (other, ingredient) in pairs:
            return amount

    ls.append(ingredient)
    amount += combinations(ls, ingredient + 1)
    ls.pop()

    return amount

print(combinations([], 1))


