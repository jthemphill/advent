import functools
import sys


joltages = []
for line in sys.stdin:
    joltages.append(int(line.rstrip()))
joltages.sort()
joltages.append(joltages[-1] + 3)


@functools.lru_cache()
def nways(joltage=0, i=0):
    if i == len(joltages):
        return 1 if joltage == joltages[-1] else 0
    if joltages[i] - joltage > 3:
        return 0
    return nways(joltage, i + 1) + nways(joltages[i], i + 1)


print(nways())
