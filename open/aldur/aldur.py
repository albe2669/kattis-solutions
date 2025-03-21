from sys import stdout

input = iter(open(0).read().splitlines())

_ = next(input)
youngest = int(next(input))

for n in input:
    youngest = min(youngest, int(n))

stdout.write(f"{youngest}\n")
