import sys

FULL = '#'
EMPTY = 'L'
FLOOR = '.'
TOLERANCE = 5

UL = 0
U = 1
UR = 2
L = 3
R = 4
DL = 5
D = 6
DR = 7


def next_adj(seats):
    new = []
    for r in range(len(seats)):
        new.append([])
        for c, seat in enumerate(seats[r]):
            new[r].append(seat)
            nadj = 0
            for dr in [-1, 0, 1]:
                if not 0 <= r+dr < len(seats):
                    continue
                for dc in [-1, 0, 1]:
                    if dr == 0 and dc == 0:
                        continue
                    if not 0 <= c+dc < len(seats[r]):
                        continue
                    nadj += 1 if seats[r+dr][c+dc] == FULL else 0
            if seat == EMPTY and nadj == 0:
                new[r][c] = FULL
            elif seat == FULL and nadj >= 4:
                new[r][c] = EMPTY
    return new


def next_ray(seats, rays):
    new = [row[:] for row in seats]
    for r, row in enumerate(seats):
        for c, seat in enumerate(row):
            if seat == FLOOR:
                continue
            nseen = 0
            for dir in range(8):
                seen = rays[r][c][dir]
                if seen is not None:
                    sr, sc = seen
                    if seats[sr][sc] == FULL:
                        nseen += 1
            if nseen >= 5:
                new[r][c] = EMPTY
            elif nseen == 0:
                new[r][c] = FULL
    return new


def make_rays(seats):
    rays = [[[None for _dir in range(8)] for _seat in row] for row in seats]
    for r in range(len(seats)):
        for c in range(len(seats[0])):
            for dir, (dr, dc) in enumerate([(-1, -1), (-1, 0), (-1, 1), (0, -1)]):
                nr, nc = r + dr, c + dc
                # ask our neighbor in that direction where the closest seat is
                if 0 <= nr < len(seats) and 0 <= nc < len(seats[0]):
                    if seats[nr][nc] == FLOOR:
                        rays[r][c][dir] = rays[nr][nc][dir]
                    else:
                        rays[r][c][dir] = (nr, nc)
                # Tell the closest seat in that direction that we are the closest seat in the opposite direction
                if seats[r][c] != FLOOR and rays[r][c][dir] is not None:
                    nr, nc = rays[r][c][dir]
                    rays[nr][nc][7 - dir] = (r, c)
    return rays


def count(seats):
    cnt = 0
    for row in seats:
        for seat in row:
            if seat == FULL:
                cnt += 1
    return cnt


seats = []
for line in sys.stdin:
    seats.append(list(line.rstrip()))
rays = make_rays(seats)
print(rays)
while 1:
    new_seats = next_ray(seats, rays)
    print('\n'.join([''.join(r) for r in seats]))
    print()
    if new_seats == seats:
        break
    seats = new_seats
print(count(seats))
