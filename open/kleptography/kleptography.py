n, m = map(int, input().split())
plain = input()[::-1]

ciphertext = list(input()[::-1])
ciphertext_i = 0

full_key = []

while ciphertext_i < n:
    full_key.append(
        (ord(ciphertext[ciphertext_i]) - ord(plain[ciphertext_i])) % 26 + 97)
    ciphertext[ciphertext_i] = plain[ciphertext_i]
    ciphertext_i += 1

while ciphertext_i < m:
    full_key.append(
        (ord(ciphertext[ciphertext_i]) - full_key[ciphertext_i - n]) % 26 + 97)
    ciphertext[ciphertext_i] = chr(full_key[ciphertext_i - n])
    ciphertext_i += 1
print(''.join((ciphertext)[::-1]))
