import re
import sys

CATEGORY = 0
MINE = 1
NEARBY = 2

category_re = re.compile(r'(.+): (\d+)-(\d+) or (\d+)-(\d+)')

state = CATEGORY
categories = {}
mine = []
nearby = []


def in_range(field):
    return any([any([a <= field <= b for a, b in r]) for r in categories.values()])


def is_valid(ticket):
    return all([in_range(field) for field in ticket])


for line in sys.stdin:
    line = line.rstrip()
    if not line:
        continue
    if state == CATEGORY:
        if line == 'your ticket:':
            state = MINE
            continue
        m = category_re.match(line)
        assert m
        name, ranges = m[1], [(int(m[2]), int(m[3])),
                              (int(m[4]), int(m[5]))]
        categories[name] = ranges
    elif state == MINE:
        if line == 'nearby tickets:':
            state = NEARBY
            continue
        mine = [int(x) for x in line.split(',')]
    else:
        assert state == NEARBY
        nearby.append([int(x) for x in line.split(',')])

assert len(categories) == len(mine)

valid_nearby = [mine] + [ticket for ticket in nearby if is_valid(ticket)]


def is_plausible_range(i, ranges):
    return all([any([a <= ticket[i] <= b for a, b in ranges]) for ticket in valid_nearby])


possible = []
for name, ranges in categories.items():
    possible.append((name, [i for i in range(len(categories))
                            if is_plausible_range(i, ranges)]))

possible.sort(key=lambda t: len(t[1]))

def cover(slots, unplaced):
    assert len(slots) == len(possible)
    assert sum(slots) == len(unplaced)

    if not unplaced:
        return [[]]
    # if len(unplaced) == 1:
    #     name, positions = unplaced[0]
    #     return [(name, pos) for pos in positions if slots[pos]]

    answers = []
    name, positions = unplaced[0]
    for pos in positions:
        if not slots[pos]:
            continue
        solns = cover(slots[:pos] + [False] + slots[pos + 1:], unplaced[1:])
        answers += [[(name, pos)] + soln for soln in solns]
    return answers

solns = cover([1] * len(possible), possible)
assert len(solns) == 1
ordering = solns[0]

prod = 1
for name, i in ordering:
    if 'departure' in name:
        prod *= mine[i]
print(prod)