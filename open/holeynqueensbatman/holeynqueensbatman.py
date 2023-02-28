import sys

def search(y):
    global cnt

    if y == n:
        cnt += 1
        return

    for x in range(0, n):
        if cols[x] or diag1[x + y] or diag2[x - y + n - 1]:
            continue
        if (x, y) in holes:
            continue 

        cols[x] = diag1[x + y] = diag2[x - y + n - 1] = True
        search(y + 1)
        cols[x] = diag1[x + y] = diag2[x - y + n - 1] = False

    return

def ln(n):
    return [False] * n

while True:
    [n,m] = [int(x) for x in sys.stdin.readline().split()]
    cnt = 0
    cols = ln(n)
    rows = ln(n)
    diag1 = ln(2 * n - 1)
    diag2 = ln(2 * n - 1)
    holes = set()

    if n == 0 and m == 0:
        break

    for i in range(m):
        (a,b) = (int(x) for x in sys.stdin.readline().split())
        holes.add((a,b))

    search(0)
    print(cnt)
