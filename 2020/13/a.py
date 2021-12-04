import sys


def euclid(a, b):
    old_r, r = a, b
    old_s, s = 1, 0
    old_t, t = 0, 1

    while r != 0:
        q = old_r // r
        old_r, r = r, old_r - q * r
        old_s, s = s, old_s - q * s
        old_t, t = t, old_t - q * t
    return (old_s, old_t), old_r


def crt(a1, n1, a2, n2):
    (m1, m2), _ = euclid(n1, n2)
    return a1 * m2 * n2 + a2 * m1 * n1


ts = None
buses = []
for line in sys.stdin:
    buses = [int(bus) if bus != 'x' else None for bus in line.split(',')]

mods = [(-i % bus, bus) for i, bus in enumerate(buses) if bus is not None]
while len(mods) > 1:
    a1, n1 = mods.pop()
    a2, n2 = mods.pop()
    n3 = n1 * n2
    a3 = crt(a1, n1, a2, n2) % n3
    print(f"CRT({a1}, {n1}, {a2}, {n2}) = {a3}")
    mods.append((a3, n3))

print(mods.pop())