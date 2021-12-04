import sys

started = False
n_ayes = 0
ayes = set()
for line in sys.stdin:
    line = line.rstrip()
    if not line:
        n_ayes += len(ayes)
        ayes = set()
        started = False
    elif not started:
        ayes = set(line)
        started = True
    else:
        ayes = ayes.intersection(line)

n_ayes += len(ayes)
print(n_ayes)