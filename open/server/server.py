import sys

t = int(sys.stdin.readline().split()[1])
tasks = [int(x) for x in sys.stdin.read().split()]

s = 0
done = 0

for task in tasks:
    if s + task > t:
        break
    
    s += task
    done += 1

print(done)


