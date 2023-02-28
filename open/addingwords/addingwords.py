import sys

word = {}
numb = {}

def handle_calc(args):
    if args[0] not in word:
        return None

    res = word[args[0]]

    for i in range(1, len(args)-1, 2):
        if args[i+1] not in word:
            return None

        value = word[args[i+1]]

        if args[i] == "+":
            res += value
        else:
            res -= value
   
    if res not in numb:
        return None
    return res

for l in sys.stdin.readlines():
    commands = l.split()
    
    if commands[0] == "def":
        if commands[1] in word.keys():
            del numb[word[commands[1]]]
        num = int(commands[2])
        word[commands[1]] = num
        numb[num] = commands[1]

    elif commands[0] == "calc":
        args = commands[1:]
        res = handle_calc(args)
        if res == None:
            print(" ".join(args) + " unknown")
        else:
            print(" ".join(args) + " " + numb[res])

    else:
        word.clear()
        numb.clear()



