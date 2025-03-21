from sys import stdout

input = iter(open(0).read().splitlines())

n = int(next(input))

if n == 0:
    stdout.write(f"{n}\n")
elif n == 7:
    stdout.write(f"{n}\n")
else:
    stdout.write(f"{n + 1}\n")
