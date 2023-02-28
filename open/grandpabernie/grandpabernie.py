import sys
from collections import defaultdict

countries = defaultdict(lambda: [])

for _ in range(0, int(sys.stdin.readline())):
    [country, year] = sys.stdin.readline().split()

    countries[country].append(int(year))

for country, _ in countries.items():
    countries[country].sort()

for _ in range(0, int(sys.stdin.readline())):
    [country, number] = sys.stdin.readline().split()
    print(countries[country][int(number) - 1])



