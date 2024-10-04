from sys import stdout

heroes = {
    "Shadow": {
        "energy_cost": 4,
        "power_level": 6,
    },
    "Gale": {
        "energy_cost": 3,
        "power_level": 5,
    },
    "Ranger": {
        "energy_cost": 2,
        "power_level": 4,
    },
    "Anvil": {
        "energy_cost": 5,
        "power_level": 7,
    },
    "Vexia": {
        "energy_cost": 2,
        "power_level": 3,
    },
    "Guardian": {
        "energy_cost": 6,
        "power_level": 8,
    },
    "Thunderheart": {
        "energy_cost": 5,
        "power_level": 6,
        "ability": "Phalanx - If the location this card is played at has 4 friendly cards, including this one, then its power is doubled. Note that other buffs the card receives are not doubled."
    },
    "Frostwhisper": {
        "energy_cost": 1,
        "power_level": 2,
    },
    "Voidclaw": {
        "energy_cost": 1,
        "power_level": 3,
    },
    "Ironwood": {
        "energy_cost": 1,
        "power_level": 3,
    },
    "Zenith": {
        "energy_cost": 6,
        "power_level": 4,
        "ability": "Centered Focus - If this card is played at the center location, +5 power."
    },
    "Seraphina": {
        "energy_cost": 1,
        "power_level": 1,
        "ability": "Celestial Healing - Seraphina grants +1 power to each friendly card at this location, other than itself. This includes other Seraphina cards."
    }
}


def process_player(played_heroes, location):
    s = sum(heroes[hero]["power_level"] for hero in played_heroes)
    for i in range(played_heroes.count("Thunderheart")):
        if len(played_heroes) == 4:
            s += heroes["Thunderheart"]["power_level"]

    for i in range(played_heroes.count("Zenith")):
        if location == 1:
            s += 5

    for i in range(played_heroes.count("Seraphina")):
        s += len(played_heroes) - 1
    return s


input = iter(open(0).read().splitlines())
p1_won = 0
p1_power = 0
p2_won = 0
p2_power = 0

for i in range(3):
    n, *played_heroes = next(input).split()
    p1_sum = process_player(played_heroes, i)
    n, *played_heroes = next(input).split()
    p2_sum = process_player(played_heroes, i)

    p1_power += p1_sum
    p2_power += p2_sum

    if p1_sum > p2_sum:
        p1_won += 1
    elif p2_sum > p1_sum:
        p2_won += 1

if p1_won > p2_won:
    stdout.write("Player 1\n")
elif p2_won > p1_won:
    stdout.write("Player 2\n")
else:
    if p1_power > p2_power:
        stdout.write("Player 1\n")
    elif p2_power > p1_power:
        stdout.write("Player 2\n")
    else:
        stdout.write("Tie\n")
