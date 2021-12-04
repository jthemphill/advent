def play(seq):
    seen = {}
    for i, n in enumerate(seq[:-1]):
        seen[n] = i + 1

    last = seq[-1]
    for i in range(len(seq), 30000000):
        next = None
        if last in seen:
            # print(f"Turn {i + 1}: {last} was last seen on turns {i} and {seen[last]}")
            next = i - seen[last]
        else:
            # print(f"Turn {i + 1}: {last} was new")
            next = 0
        # print(f"Turn {i + 1}: Appended {next}")
        seen[last] = i
        last = next
    return last

# print(play([0,3,6]))
print(play([7,12,1,0,16,2]))