import sys

def valid(minN, maxN, letter, pw):
    return (minN <= len(pw) and pw[minN - 1] == letter) ^ (maxN <= len(pw) and pw[maxN - 1] == letter)

if __name__ == '__main__':
    nvalid = 0
    for line in sys.stdin:
        parts = line.split()
        minN, maxN = (int(p) for p in parts[0].split('-'))
        letter = parts[1][0]
        pw = parts[2]

        if valid(minN, maxN, letter, pw):
            nvalid += 1

    print(f"{nvalid} valid.")