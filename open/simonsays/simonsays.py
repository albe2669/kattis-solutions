from sys import stdout

input = iter(open(0).read().splitlines())

n = int(next(input))

for _ in range(n):
    string = next(input)
    if string.startswith("Simon says"):
        stdout.write(string[10:] + "\n")
