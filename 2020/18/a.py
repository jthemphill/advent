import sys

# T: I | '('E')'
# F: T {'+' T}*
# E: F {'*' F}*


def solve_term(rui):
    if rui[0] == '(':
        e, rui = solve_expr(rui[1:])
        assert rui[0] == ')'
        return e, rui[1:]
    else:
        return int(rui[0]), rui[1:]


def solve_factor(rui):
    acc, rui = solve_term(rui)
    while rui and rui[0] == '+':
        t2, rui = solve_term(rui[1:])
        acc += t2
    return acc, rui


def solve_expr(rui):
    acc, rui = solve_factor(rui)
    while rui and rui[0] == '*':
        t2, rui = solve_factor(rui[1:])
        acc *= t2
    return acc, rui


answers = []
exprs = []
for line in sys.stdin:
    answers.append(solve_expr([c for c in line.rstrip() if c != ' ']))
print(sum([a for a, _ in answers]))
