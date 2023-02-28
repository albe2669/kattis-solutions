import sys

n = int(sys.stdin.readline())

for i in range(n):
    sounds = sys.stdin.readline().split()

    line = sys.stdin.readline()
    while line != "what does the fox say?\n":
        sound = line.split()[2]
        sounds = [x for x in sounds if x != sound]

        line = sys.stdin.readline()

    print(" ".join(sounds))
