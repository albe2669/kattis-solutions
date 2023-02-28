import sys

def run(prim):
    aux = []
    moves = 0

    while True:
        if len(prim) > 0 and len(aux) > 0:
            if prim[-1] == aux[-1]:
                prim.pop()
                aux.pop()
            else:
                aux.append(prim.pop())
        elif len(prim) > 0:
            aux.append(prim.pop())
        elif len(aux) > 0:
            return "impossible"
        else:
            return moves

        moves += 1

print(run([int(x) for x in reversed(sys.stdin.read().split()[1:])]))
