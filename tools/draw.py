import random

final = []

retry_because_picked_own_name = True

while retry_because_picked_own_name:
    retry_because_picked_own_name = False
    names = {'Sharon', 'George', 'Heather', 'Jennifer', 'Ashley', 'Sam'}
    hat = list(names)

    for name in names:
        pick = random.choice(hat)
        hat.remove(pick)

        if name == pick:
            retry_because_picked_own_name = True
            final.clear()
            break

        else:
            final.append((name, pick))

for name, pick in final:
    print(name, '->', pick)