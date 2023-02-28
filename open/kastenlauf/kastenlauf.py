import sys
from collections import defaultdict

class Graph:
    def __init__(self):
        self.edges = {}

    def add_vertex(self, vertex):
        self.edges[vertex] = set()
        x,y = vertex

        for a,b in self.edges.keys():
            if a == x and b == y:
                continue

            if abs(a-x) + abs(b-y) <= (50*20):
                self.edges[(a,b)].add(vertex)
                self.edges[vertex].add((a,b))
        
    def has_path(self, start, end):
        xs, ys = start
        xe, ye = end

        visited = set()
        stack = [(xs, ys)]

        while stack:
            cx, cy = stack.pop()

            if cx == xe and cy == ye:
                return True

            if (cx,cy) in visited:
                continue

            visited.add((cx,cy))

            for x,y in self.edges[(cx,cy)]:
                if (x,y) not in visited:
                    stack.append((x,y))

        return False

t = int(sys.stdin.readline())

for i in range(0, t):
    n = int(sys.stdin.readline())
    g = Graph()

    home = tuple(int(x) for x in sys.stdin.readline().split())
    g.add_vertex(home)

    for j in range(0, n):
        g.add_vertex(tuple(int(x) for x in sys.stdin.readline().split()))

    end = tuple(int(x) for x in sys.stdin.readline().split())
    g.add_vertex(end)

    if g.has_path(home, end):
        print("happy")
    else:
        print("sad")
