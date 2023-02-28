import sys

def test_case(n):
    stack1 = 0
    stack2 = 0

    for i in range(0, n):
        [command, value] = sys.stdin.readline().split()
        value = int(value)

        if command == "DROP":
            stack2 += value
            print(f"DROP 2 {value}")
        elif command == "TAKE":
            from_one = min(stack1, value)
            if from_one != 0:
                value -= from_one
                stack1 -= from_one

                print(f"TAKE 1 {from_one}")
            if value != 0:
                print(f"MOVE 2->1 {stack2}")
                stack1 += stack2 - value
                stack2 = 0
                print(f"TAKE 1 {value}")

t = int(sys.stdin.readline())
while t != 0:
    test_case(t)
    print("")
    t = int(sys.stdin.readline())


