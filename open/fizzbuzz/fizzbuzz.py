import sys

[x, y, n] = [int(x) for x in sys.stdin.readline().split()]

for i in range(1, n+1):
    divy = i % y == 0
    divx = i % x == 0

    if divx and divy:
        print("FizzBuzz")
    elif divx:
        print("Fizz")
    elif divy:
        print("Buzz")
    else:
        print(i)
