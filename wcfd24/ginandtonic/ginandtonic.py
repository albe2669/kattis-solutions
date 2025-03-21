from collections import defaultdict


def dfs(graph, start, dest, visited: set[str]):  # returns path to dest
    stack = [start]
    parent = {start: None}
    while stack:
        current = stack.pop()

        for u in graph[current]:
            if graph[current][u] > 0 and u not in visited:
                visited.add(u)
                stack.append(u)
                parent[u] = current

                if u == dest:
                    p = []
                    current_vertex = dest
                    while start != current_vertex:
                        p.append((parent[current_vertex], current_vertex))
                        current_vertex = parent[current_vertex]
                    return p

    return None


def flow(orggraph, src, dest):
    graph = defaultdict(lambda: defaultdict(int))
    maxcapacity = 0
    for u, d in orggraph.items():
        for v, c in d.items():
            graph[u][v] = c
            maxcapacity = max(maxcapacity, c)

    current_flow = 0
    mincap = maxcapacity
    while True:
        visited = set()
        path = dfs(graph, src, dest, visited)
        if not path:
            return current_flow
        saturation = min(graph[u][v] for u, v in path)
        # for i in range(len(p)-1):
        #     assert(p[i][0] == p[i+1][1])
        # print(current_flow,saturation,file=sys.stderr)#,[f"{u[0]}-{u[1]}:{inp[u[0]][u[1]]}:{graph[u][v]}" for u,v in p if u[2]==0])
        current_flow += saturation
        for u, v in path:
            graph[u][v] -= saturation
            graph[v][u] += saturation


if __name__ == "__main__":
    graph = defaultdict(lambda: defaultdict(int))

    input = iter(open(0).read().splitlines())
    source = "0"
    sink = "1"

    n = int(next(input))
    pallergens = defaultdict(lambda: (0, []))
    allergen_lookup = {}
    for _ in range(n):
        name, _, *allergens = next(input).split(" ")
        nallergens = []
        for a in allergens:
            if a not in allergen_lookup:
                allergen_lookup[a] = len(allergen_lookup)
            nallergens.append(allergen_lookup[a])

        string = ",".join([str(i) for i in sorted(nallergens)])

        pallergens[string] = (
            pallergens[string][0] + 1,
            nallergens,
        )

    for p in pallergens:
        graph[p + "-in"][p + "-out"] = pallergens[p][0]

    g = int(next(input))
    for _ in range(g):
        name, u, _, *allergens = next(input).split(" ")
        allergens = [allergen_lookup[a] for a in allergens]

        graph[source][name] = int(u)
        for p in pallergens:
            for a in allergens:
                if a in pallergens[p][1]:
                    break
            else:
                graph[name][p + "-in"] = 1000

    t = int(next(input))
    for _ in range(t):
        name, u, _, *allergens = next(input).split(" ")
        allergens = [allergen_lookup[a] for a in allergens]

        graph[name][sink] = int(u)
        for p in pallergens:
            for a in allergens:
                if a in pallergens[p][1]:
                    break
            else:
                graph[p + "-out"][sink] = 1000

    flow = flow(graph, source, sink)
    print(flow)
