[K, N] = [int(x) for x in input().split()]

# For each k -1


def funktion(k, n, direction, indent):
    # print(" " * indent, k, n, direction)
    if k == 0 and n == 0:
        return "YES"
    if k == 0 and n != 0:
        return None
    if n == 0:
        return "NO"

    results = set()
    for i in range(1, n + 1):
        dir = direction if i % 2 == 0 else -direction
        res = funktion((k+dir) % K, n-i, dir, indent+1)
        if res is not None:
            results.add(res)
        # print(" " * indent, "===", results)
    # print(" " * indent, results)

    if len(results) > 1:
        return "MAYBE"
    else:
        return results.pop()


if __name__ == "__main__":
    print(funktion(1, N, 1, 0))
