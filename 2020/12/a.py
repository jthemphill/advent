import sys


def turn_right(wp):
    x, y = wp
    return y, -x


def turn_left(wp):
    x, y = wp
    return -y, x


def move_waypoint(wp, dir, mag):
    x, y = wp
    if dir == 'N':
        return x, y + mag
    elif dir == 'E':
        return x + mag, y
    elif dir == 'S':
        return x, y - mag
    else:
        assert dir == 'W'
        return x - mag, y


def run(instrs):
    pos = (0, 0)
    wp = (10, 1)
    for dir, mag in instrs:
        if dir in ['N', 'E', 'S', 'W']:
            wp = move_waypoint(wp, dir, mag)
        elif dir == 'L':
            for _ in range(0, mag, 90):
                wp = turn_left(wp)
        elif dir == 'R':
            for _ in range(0, mag, 90):
                wp = turn_right(wp)
        elif dir == 'F':
            x, y = pos
            wx, wy = wp
            pos = x + mag * wx, y + mag * wy
    return pos, wp


instrs = []
for line in sys.stdin:
    instrs.append((line[0], int(line[1:])))

print(run(instrs))
