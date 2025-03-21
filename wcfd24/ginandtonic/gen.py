import random

n = 1000
g = 100
t = 100

allergens = [
    "almonds",
    "nuts",
    "quinine",
    "dmwal",
    "dgmkl",
    "gnkldn",
    "gkdfn",
    "fgmsklg",
]

test = [f"{n}"]

for _ in range(n):
    name = "".join(random.choices("abcdefghijklmnopqrstuvwxyz", k=5))
    allergen = random.choices(allergens, k=random.randint(0, 10))
    test.append(f"{name} {len(allergen)} {' '.join(allergen)}")

test.append(f"{g}")
for _ in range(g):
    name = "".join(random.choices("abcdefghijklmnopqrstuvwxyz", k=5))
    u = random.randint(1, n)
    allergen = random.choices(allergens, k=random.randint(0, 10))
    test.append(f"{name} {u} {len(allergens)} {' '.join(allergen)}")

test.append(f"{t}")
for _ in range(t):
    name = "".join(random.choices("abcdefghijklmnopqrstuvwxyz", k=5))
    u = random.randint(1, n)
    allergen = random.choices(allergens, k=random.randint(0, 10))
    test.append(f"{name} {u} {len(allergens)} {' '.join(allergen)}")

print("\n".join(test))
