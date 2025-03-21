from sys import stdout

input = iter(open(0).read().splitlines())

no_y = 0
wi_y = 0

for char in next(input):
    if char in "aeiou":
        no_y += 1
    if char == "y":
        wi_y += 1

stdout.write(f"{no_y} {no_y + wi_y}\n")
