import sys


def step(instrs, instrp, acc):
    if not (0 <= instrp < len(instrs)):
        return None
    op, arg = instrs[instrp]
    if op == 'acc':
        return instrp + 1, acc + arg
    elif op == 'jmp':
        return instrp + arg, acc
    else:
        assert op == 'nop'
        return instrp + 1, acc


class VM:
    def __init__(self, instrs):
        self.instrs = instrs
        self.instrp = 0
        self.acc = 0

    def step(self):
        if not 0 <= self.instrp < len(self.instrs):
            return self.acc
        self.instrp, self.acc = step(self.instrs, self.instrp, self.acc)
        return None

def exec(instrs):
    vm = VM(instrs)
    while True:
        acc = vm.step()
        if acc is not None:
            return acc

def print_execution(instrs):
    vm = VM(instrs)
    t = 0
    output = [(op, arg, None) for (op, arg) in vm.instrs]
    while True:
        t += 1
        op, arg, lastT = output[vm.instrp]
        if lastT is not None:
            break
        output[vm.instrp] = (op, arg, t)
        vm.step()

    for i, (op, arg, t) in enumerate(output):
        print(f"{i:3} | {op} {arg:+4} | {'' if t is None else t:3}")


def find_bug(instrs, seen=None, fix=None, instrp=0):
    if seen is None:
        seen = set()
    if instrp in seen:
        return None
    seen.add(instrp)

    # Base case: see if we terminate by just running the current instruction
    unfixres = step(instrs, instrp, 0)
    if unfixres is None:
        assert fix is not None
        return fix

    # If we haven't fixed an instruction yet, try fixing the current instruction and recurse
    if not fix:
        fixedinstrs = None
        op, arg = instrs[instrp]
        if op == 'jmp':
            fixedinstrs = (instrs[:instrp] +
                           [('nop', arg)] + instrs[instrp + 1:])
            assert len(fixedinstrs) == len(instrs)
        elif op == 'nop':
            fixedinstrs = (instrs[:instrp] +
                           [('jmp', arg)] + instrs[instrp + 1:])
            assert len(fixedinstrs) == len(instrs)
        if fixedinstrs is not None:
            fixres = step(fixedinstrs, instrp, 0)
            if fixres is None:
                return instrp
            instrp, _ = fixres
            finalfix = find_bug(fixedinstrs, seen.copy(), instrp, instrp)
            if finalfix is not None:
                assert finalfix == instrp
                return finalfix

    # Try recursing without fixing this instruction
    instrp, _ = unfixres
    return find_bug(instrs, seen.copy(), fix, instrp)


instrs = []
for line in sys.stdin:
    line = line.rstrip()
    op, argstr = line.split()
    arg = int(argstr[1:])
    if argstr[0] == '-':
        arg = -arg
    instrs.append((op, arg))

# print(find_bug(instrs))
# print(exec(instrs))
