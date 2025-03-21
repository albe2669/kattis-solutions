from collections import defaultdict
from sys import stdout

input = iter(open(0).read().splitlines())

n = int(next(input))
k = int(next(input))

already_matched = 0

cards = [""] * (n + 1)

for i in range(k):
    c1, c2, p1, p2 = next(input).split()

    cards[int(c1)] = p1
    cards[int(c2)] = p2

    if p1 == p2:
        already_matched += 1

for_sure = 0
unsure = 0
unknown = 0

for i in range(1, n + 1):
    found = False

    if cards[i] == "":
        unknown += 1
        continue

    for j in range(i + 1, n + 1):
        if cards[i] == cards[j]:
            for_sure += 1
            found = True
            break
    if not found:
        unsure += 1

unsure -= for_sure

res = for_sure - already_matched
if unsure == unknown:
    res += unsure
if unsure == 0 and unknown == 2:
    res += 1

stdout.write(f"{res}\n")
