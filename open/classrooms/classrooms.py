from io import BytesIO
from os import read, fstat
from sys import stdout
from bisect import bisect_right

input = BytesIO(read(0, fstat(0).st_size)).readline

[n, k] = [int(x) for x in input().decode().split()]

intervals = []
for i in range(n):
    [a, b] = [int(x) for x in input().decode().split()]
    intervals.append((a, b))

intervals.sort(key=lambda x: (x[1], x[0]))

count = 0
classrooms = [0] * k

for interval in intervals:
    # Find the index of the last end that is smaller than the start of the interval
    upper_bound = bisect_right(classrooms, interval[0])

    # print(interval, classrooms, upper_bound, len(classrooms))
    # If starts before all end times
    if upper_bound != 0:
        classrooms.pop(upper_bound-1)
        classrooms.append(interval[1] + 1)
        count += 1

stdout.write(str(count) + '\n')
