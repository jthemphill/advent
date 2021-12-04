import sys

def seat_r(tkt):
    assert len(tkt) == 7
    lo = 0
    hi = 127
    for c in tkt:
        mid = (lo + hi) // 2
        if c == 'F':
            hi = mid
        else:
            assert c == 'B'
            lo = mid + 1
    assert lo == hi
    return lo

def seat_c(tkt):
    assert len(tkt) == 3
    lo = 0
    hi = 7
    for c in tkt:
        mid = (lo + hi) // 2
        if c == 'L':
            hi = mid
        else:
            assert c == 'R'
            lo = mid + 1
    assert lo == hi
    return lo

def get_sid(tkt):
    r = seat_r(tkt[:7])
    c = seat_c(tkt[7:10])
    return r * 8 + c

filled = []
for line in sys.stdin:
    tkt = line.rstrip()
    filled.append(get_sid(tkt))

filled.sort()
for i in range(len(filled) - 1):
    if filled[i] + 2 == filled[i + 1]:
        print(f"{filled[i]} - {filled[i + 1]}")
        sys.exit(0)

sys.exit(1)