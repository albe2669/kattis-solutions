import sys


def calc(sprinklers, l):
    count = 0
    i = 0
    old = 0

    n = len(sprinklers)

    while True:
        longest_range = -1

        while i < n and sprinklers[i][0] <= old:
            longest_range = max(longest_range, sprinklers[i][1])
            i += 1

        if longest_range == -1:
            return -1

        count += 1

        if longest_range >= l:
            return count

        old = longest_range


data = sys.stdin.readlines()

line_iter = iter(data)

while line_iter:
    try:
        line = next(line_iter).strip()
        if not line:
            break
        n, l, w = map(int, line.split())
    except StopIteration:
        break

    half = (w / 2) ** 2
    sprinklers = []
    for i in range(n):
        x, r = map(int, next(line_iter).split())

        if w > 2 * r:
            continue

        d = (r ** 2 - half) ** 0.5
        sprinklers.append((x - d, x + d))

    print(calc(sorted(sprinklers), l))
