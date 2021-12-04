import sys

def numTrees(rows, rskip, dskip):
    col = 0
    ntrees = 0
    for row in rows[::dskip]:
        if row[col] == '#':
            ntrees += 1
        col += rskip
        col %= len(row)
    return ntrees

rows = []
for line in sys.stdin:
    rows.append(line.rstrip())
    print(rows[-1])

answer = 1
for rskip, dskip in [(1,1), (3,1), (5,1), (7,1), (1,2)]:
    ntrees = numTrees(rows, rskip, dskip)
    print(f"{(rskip, dskip)}: {ntrees}")
    answer *= ntrees

print(answer)