def lcs(s1, s2):
    matrix = [["" for x in range(len(s2))] for x in range(len(s1))]
    for i in range(len(s1)):
        for j in range(len(s2)):
            if s1[i] == s2[j]:
                if i == 0 or j == 0:
                    matrix[i][j] = s1[i]
                else:
                    matrix[i][j] = matrix[i-1][j-1] + s1[i]
            else:
                matrix[i][j] = max(matrix[i-1][j], matrix[i][j-1], key=len)

    cs = matrix[-1][-1]

    return len(cs), cs


a = input()
b = input()
sequence = lcs(a, b)[1]

i, j = 0, 0
indexes = []
out = ""
for c in sequence:
    while a[i] != c:
        i += 1
    while b[j] != c:
        j += 1

    old_index = indexes[-1] if indexes else (-1, -1)
    out += a[old_index[0]+1:i] + b[old_index[1]+1:j] + c
    indexes.append((i, j))

    i += 1
    j += 1

index = indexes[-1] if indexes else (-1, -1)
if index[0]+1 < len(a):
    out += a[index[0]+1:]
if index[1]+1 < len(b):
    out += b[index[1]+1:]

print(out)
