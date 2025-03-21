from sys import stdout


def solve():
    input = iter(open(0).read().splitlines())

    n = int(next(input))
    turn = False
    lasta = 10
    lastb = 10

    for i in range(n):
        a, b = map(int, next(input).split())
        change = 2
        if i == 0:
            if b != 10:
                turn = True

            change = 1

        if not turn and b != lastb:
            return "invalid"
        if turn and a != lasta:
            return "invalid"

        if abs(a - lasta) > change or abs(b - lastb) > change:
            return "invalid"

        if a > lasta or b > lastb:
            return "invalid"

        if a == 0 or b == 0:
            if i != n - 1:
                return "invalid"
            return "finished"

        lasta = a
        lastb = b
        turn = not turn
    return "ongoing"


d = solve()
stdout.write(f"{d}\n")
