import sys

sys.stdin.readline()
low = len([x for x in sys.stdin.readline().split() if int(x) < 0])
print(low)
