import re
import sys

GOLD = 'shiny gold'

decl_re = re.compile(r'^(.*) bags contain (.*)\.$')
inner_bag_re = re.compile(r'(\d+) (.*) bags?')

bag_defs = {}
for line in sys.stdin:
    line = line.rstrip()
    m = decl_re.match(line)
    if not m:
        print(f"Not parseable: {line}")
    outer_bag = m[1]
    inner_bag_str = m[2]

    bag_def = {}
    if inner_bag_str != 'no other bags':
        for inner_bag in inner_bag_str.split(', '):
            m = inner_bag_re.match(inner_bag)
            bag_def[m[2]] = int(m[1])
    bag_defs[outer_bag] = bag_def

def has_gold(bag):
    return bag == GOLD or any([has_gold(inner_bag) for inner_bag in bag_defs[bag].keys()])

def num_bags(bag):
    bag_def = bag_defs[bag]
    if not bag_def:
        return 1
    return 1 + sum([num_bags(inner_bag) * nbags for inner_bag, nbags in bag_def.items()])

assert GOLD in bag_defs
print(f"{num_bags(GOLD) - 1} other bags.")