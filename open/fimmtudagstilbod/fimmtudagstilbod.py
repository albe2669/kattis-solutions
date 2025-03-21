from sys import stdout

input = iter(open(0).read().splitlines())

year = int(next(input))
if year <= 2020:
    x = 1000
else:
    x = 1000 + (year - 2020) * 100

stdout.write(f"{x}\n")
