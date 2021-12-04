import typing as ty
import sys


Atom = ty.Union[int, str]
Clause = ty.List[Atom]
Rule = ty.List[Clause]


def parse_tok(tok: str) -> Atom:
    if tok[0] == '"':
        assert len(tok) == 3
        assert tok[-1] == '"'
        return tok[1]
    else:
        return int(tok)


def parse_rule(rule_toks: ty.List[str]) -> ty.Tuple[int, Rule]:
    assert rule_toks[0][-1] == ':'
    rule_id = int(rule_toks[0][:-1])
    clauses: Rule = []
    cur_clause: Clause = []
    for tok in rule_toks[1:]:
        if tok == '|':
            clauses.append(cur_clause)
            cur_clause = []
        else:
            cur_clause.append(parse_tok(tok))
    clauses.append(cur_clause)
    return (rule_id, clauses)


rules: ty.Dict[int, Rule] = {}
for line in sys.stdin:
    line = line.rstrip()
    if not line:
        break
    k, rule = parse_rule(line.split())
    rules[k] = rule

strings: ty.List[str] = []
for line in sys.stdin:
    strings.append(line.rstrip())


def matches_clause(rui: str, clause: Clause) -> ty.List[str]:
    if not clause:
        return [rui]
    if not rui:
        return []
    sym = clause[0]
    if type(sym) is str:
        assert len(sym) == 1
        return matches_clause(rui[1:], clause[1:]) if rui[0] == sym else []
    assert type(sym) is int
    next_rule = rules[sym]
    all_match_ruis: ty.List[str] = []
    for r_clause in next_rule:
        match_ruis = matches_clause(rui, r_clause)
        if match_ruis is not None:
            for match_rui in match_ruis:
                all_match_ruis += matches_clause(match_rui, clause[1:])
    return all_match_ruis


print(rules)
zero_clauses = rules[0]
assert len(zero_clauses) == 1
parsings = [(s, matches_clause(s, zero_clauses[0])) for s in strings]
print(parsings)
print(sum([any([rui == '' for rui in p]) for _, p in parsings]))
