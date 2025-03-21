from sys import stdout

input = iter(open(0).read().splitlines())

n = int(next(input))
first_layer = "  "
second_layer = "  "
third_layer = "H-"

first_layer += " ".join(["H" for _ in range(n)]) + " "
second_layer += " ".join(["|" for _ in range(n)]) + " "
third_layer += "-".join(["C" for _ in range(n)]) + "-OH"


stdout.write(
    f"{first_layer}\n{second_layer}\n{third_layer}\n{second_layer}\n{first_layer}\n"
)
