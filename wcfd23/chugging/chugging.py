n = int(input())

[ta, da] = [int(x) for x in input().split()]
[tb, db] = [int(x) for x in input().split()]

time_a = 0
time_b = 0

for i in range(n):
    time_a += ta + (da * i)
    time_b += tb + (db * i)

if time_a < time_b:
    print("Alice")
elif time_a > time_b:
    print("Bob")
else:
    print("=")
