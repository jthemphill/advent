import re
import sys

MASKSET = 0
MEMSET = 1

mask_re = re.compile(r'mask = ([X10]{36})')
memset_re = re.compile(r'mem\[(\d+)\] = (\d+)')


instrs = []
for line in sys.stdin:
    m = memset_re.match(line)
    if m:
        instrs.append((MEMSET, (int(m[1]), int(m[2]))))
        continue
    m = mask_re.match(line)
    if m:
        instrs.append((MASKSET, m[1]))
        continue


def make_floats(float_bits):
    if not float_bits:
        return [(~0, 0)]
    ret = []
    next_floats = make_floats(float_bits[1:])
    ret += [(off & ~(1 << float_bits[0]), on) for off, on in next_floats]
    ret += [(off, on | (1 << float_bits[0])) for off, on in next_floats]
    assert len(ret) == 2**len(float_bits)
    return ret


def apply_floats(src, floats):
    return [f & src for f in floats]


or_mask = 0
zero_mask = 0
float_bits = []
mem = {}
for j, (instr_type, instr_args) in enumerate(instrs):
    if instr_type == MASKSET:
        or_mask = 0
        float_bits = []
        assert type(instr_args) is str
        assert len(instr_args) == 36
        for i, c in enumerate(instr_args):
            if c == '1':
                or_mask |= 1 << (35-i)
            elif c == 'X':
                float_bits.append(35-i)
            else:
                assert c == '0'
                zero_mask |= 1 << (35-i)
        floats = [(off, on | or_mask) for off, on in make_floats(float_bits)]
        print(floats)

    else:
        assert instr_type == MEMSET
        src, val = instr_args
        for off, on in floats:
            new_src = (src | on) & off
            print(src, new_src)
            mem[new_src] = val

# print(or_mask)
# print(and_mask)
print(mem)
print(sum(mem.values()))
