import sys

print(
        len(set([int(x) % 42 for x in sys.stdin.read().splitlines()]))
)
