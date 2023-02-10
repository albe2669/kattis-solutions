def check(n):
    knowledge = set()
    for i in range(n):
        line = input()
        if line[0] == '-':
            knowledge.add(line[3:])
        else:
            assumptions, conclusion = line.split(' -> ')
            if all(assumption in knowledge for assumption in assumptions.split()):
                knowledge.add(conclusion)
            else:
                return i+1
    return 'correct'

if __name__ == "__main__":
    print(check(int(input())))