import sys

CYCLES = 6
ACTIVE = '#'
INACTIVE = '.'

neighbors = []
for dx in (-1, 0, 1):
    for dy in (-1, 0, 1):
        for dz in (-1, 0, 1):
            for dw in (-1, 0, 1):
                if not (dx == 0 and dy == 0 and dz == 0 and dw == 0):
                    neighbors.append((dx, dy, dz, dw))
assert len(neighbors) == 80

orig = []
for line in sys.stdin:
    orig.append([])
    for c in line.rstrip():
        orig[-1].append(c)

cube = {}
for y in range(-CYCLES-1, len(orig) + CYCLES + 2):
    for x in range(-CYCLES-1, len(orig[0]) + CYCLES + 2):
        for z in range(-CYCLES-1, CYCLES + 2):
            for w in range(-CYCLES-1, CYCLES + 2):
                cube[(x, y, z, w)] = False

for y, row in enumerate(orig):
    for x, c in enumerate(row):
        if c == ACTIVE:
            cube[(x, y, 0, 0)] = True

for i in range(CYCLES):
    # for z in range(-1, 3):
    #     print(f" z = {z}")
    #     for y in range(-CYCLES, len(orig) + CYCLES):
    #         row = ''
    #         for x in range(-CYCLES, len(orig[0]) + CYCLES):
    #             if cube[(x, y, z)]:
    #                 row += ACTIVE
    #             else:
    #                 row += INACTIVE
    #         print(row)

    next_gen = cube.copy()
    for y in range(-CYCLES, len(orig) + CYCLES + 1):
        for x in range(-CYCLES, len(orig[0]) + CYCLES + 1):
            for z in range(-CYCLES, CYCLES + 1):
                for w in range(-CYCLES, CYCLES + 1):
                    num_neighbors = sum([cube[(x + dx, y + dy, z + dz, w + dw)]
                                         for dx, dy, dz, dw in neighbors])
                    if cube[(x, y, z, w)]:
                        next_gen[(x, y, z, w)] = num_neighbors in (2, 3)
                    else:
                        next_gen[(x, y, z, w)] = num_neighbors == 3
    print(f"Generation {i+1}: ({sum(next_gen.values())} active)")
    cube = next_gen
