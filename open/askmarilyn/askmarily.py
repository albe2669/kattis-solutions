from random import choice

rounds = 1000

for _ in range(rounds):
    choices = ['A', 'B', 'C']
    ch = choice(choices)
    choices.remove(ch)

    print(ch)

    choice_hint, bottles_achieved = input().split()
    if bottles_achieved == '1':
        ch = choice_hint
    else:
        if choice_hint in choices:
            choices.remove(choice_hint)
        ch = choices.pop()

    print(ch)
    sanity_check = input().split()
    assert int(ch == sanity_check[1]) == int(sanity_check[0])
