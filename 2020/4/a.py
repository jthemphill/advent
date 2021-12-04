import re
import sys

yr_re = re.compile(r'^\d\d\d\d$')
hgt_re = re.compile(r'^(\d{2,3})(cm|in)$')
hcl_re = re.compile(r'^#[0-9a-f]{6}$')
pid_re = re.compile(r'^\d{9}$')
ecl_set = set(['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'])

def validyear(passport, k, minY, maxY):
    if k not in passport:
        return False
    yr = passport[k]
    if not yr_re.match(yr):
        return False
    return minY <= int(yr) <= maxY

def valid(passport):
    if not validyear(passport, 'byr', 1920, 2002):
        return False
    if not validyear(passport, 'iyr', 2010, 2020):
        return False
    if not validyear(passport, 'eyr', 2020, 2030):
        return False

    if 'hgt' not in passport:
        return False
    m = hgt_re.match(passport['hgt'])
    if not m:
        return False
    if m[2] == 'cm':
        if not 150 <= int(m[1]) <= 193:
            return False
    else:
        assert m[2] == 'in'
        if not 59 <= int(m[1]) <= 76:
            return False

    if 'hcl' not in passport:
        return False
    hcl = passport['hcl']
    if not hcl_re.match(hcl):
        return False

    if 'ecl' not in passport:
        return False
    if passport['ecl'] not in ecl_set:
        return False

    if 'pid' not in passport:
        return False
    pid = passport['pid']
    if not pid_re.match(pid):
        return False

    return True


passports = []
cur = {}
for line in sys.stdin:
    if line == '\n':
        passports.append(cur)
        cur = {}
    else:
        parts = line.split()
        for k, v in [part.split(':') for part in parts]:
            cur[k] = v
passports.append(cur)
print(len([p for p in passports if valid(p)]))