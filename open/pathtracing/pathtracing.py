import sys

extremes = [500, 500, 500, 500]

current = (500, 500)
coords = set()
coords.add(current)

for l in sys.stdin.readlines():
    x,y = current
    if l == "up\n":
        y += 1
    elif l == "down\n":
        y -= 1
    elif l == "left\n":
        x -= 1
    elif l == "right\n":
        x += 1

    current = (x,y)
    coords.add(current)
    extremes[0] = min(extremes[0], x) # min x
    extremes[1] = min(extremes[1], y) # min y
    extremes[2] = max(extremes[2], x) # max x
    extremes[3] = max(extremes[3], y) # max y

res = "#" * (extremes[2] - extremes[0] + 3) + "\n"

for y in reversed(range(extremes[1], extremes[3]+1)):
    res += "#"

    for x in range(extremes[0], extremes[2]+1):
        coord = (x,y)

        if coord == (500, 500):
            res += "S"
        elif coord == current:
            res += "E"
        elif coord in coords:
            res += "*"
        else:
            res += " "
    
    res += "#\n"

res += "#" * (extremes[2] - extremes[0] + 3)
print(res)
