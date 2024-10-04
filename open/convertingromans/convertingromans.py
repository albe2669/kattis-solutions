from sys import stdout

roman_numerals = {
    "I": 1,
    "V": 5,
    "X": 10,
    "L": 50,
    "C": 100,
    "D": 500,
    "M": 1000
}

input = iter(open(0).read().splitlines())
n = int(next(input))

for _ in range(n):
    largest = 0
    m = 0
    for c in reversed(next(input)):
        if roman_numerals[c] >= largest:
            m += roman_numerals[c]
            largest = roman_numerals[c]
        else:
            m -= roman_numerals[c]

    stdout.write(f"{m}\n")
