import sys
import typing as ty


def parse_coords(s: str):
    coords: ty.List[str] = []
    i = 0
    while i < len(s):
        if s[i] in ('n', 's'):
            coords.append(s[i:i+2])
            i += 2
        else:
            coords.append(s[i])
            i += 1
    return coords


movs = {
    'ne': (1, 0, -1),
    'e': (1, -1, 0),
    'se': (0, -1, 1),
    'sw': (-1, 0, 1),
    'w': (-1, 1, 0),
    'nw': (0, 1, -1),
}


def get_neighbors(pos: ty.Tuple[int, int, int]):
    return [(pos[0] + mov[0], pos[1] + mov[1], pos[2] + mov[2]) for mov in movs.values()]


flipped = set()
for line in sys.stdin:
    coords = parse_coords(line.rstrip())
    pos = [0] * 3
    for d in coords:
        mov = movs[d]
        for i in range(3):
            pos[i] += mov[i]
        assert sum(pos) == 0
    pos = tuple(pos)
    if pos in flipped:
        flipped.remove(pos)
    else:
        flipped.add(pos)


def should_flip(pos: ty.Tuple[int, int, int]):
    neighbors = [(pos[0] + mov[0], pos[1] + mov[1], pos[2] + mov[2]) for mov in movs.values()]
    num_flipped_neighbors = sum([n in flipped for n in neighbors])
    if pos in flipped:
        return 1 <= num_flipped_neighbors <= 2
    else:
        return num_flipped_neighbors == 2

for _ in range(100):
    print(len(flipped))
    next_flipped = set()
    for tile in flipped:
        if should_flip(tile):
            next_flipped.add(tile)
        for neighbor in get_neighbors(tile):
            if should_flip(neighbor):
                next_flipped.add(neighbor)
    flipped = next_flipped

print(len(flipped))