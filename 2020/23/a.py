import sys
import typing as ty


class Cup:
    def __init__(self, val: int):
        self.val = val
        self.next: ty.Optional[Cup] = None


cups = {}
head: ty.Optional[Cup] = None
tail: ty.Optional[Cup] = None
for val in [int(val) for val in next(sys.stdin).rstrip()]:
    new_cup = Cup(int(val))
    cups[int(val)] = new_cup

    if head is None:
        head = new_cup
    if tail is not None:
        tail.next = new_cup
    tail = new_cup


def to_str_from_one():
    vals = ''
    one = cups[1]
    one_tail = one.next
    assert one_tail is not None
    for _ in range(N):
        vals += str(one_tail.val)
        one_tail = one_tail.next
    return vals


assert head is not None
assert tail is not None
assert min(cups.keys()) == 1
assert max(cups.keys()) == len(cups)

N = 1_000_000
N_ITERS = 10_000_000

for i in range(len(cups) + 1, N + 1):
    new_cup = Cup(i)
    tail.next = new_cup
    cups[i] = new_cup
    tail = new_cup
tail.next = head

assert len(cups) == N
one = cups[1]
one_tail = one.next
assert one_tail is not None

for i in range(N_ITERS):
    if i % 10_000 == 0:
        print(i)
    # print(f"-- move {i + 1} --")
    # print("cups", head.val, to_str_from_one())

    # The crab picks up the three cups that are immediately clockwise of the
    # current cup.
    pickup = [head.next, head.next.next, head.next.next.next]
    # print("pickup", [cup.val for cup in pickup])

    # They are removed from the circle; cup spacing is adjusted as necessary
    # to maintain the circle.
    head.next = pickup[-1].next

    # The crab selects a destination cup: the cup with a label equal to the
    # current cup's label minus one.
    dst = head.val - 1
    while dst <= 0:
        dst += len(cups)

    # If this would select one of the cups that was just picked up, the crab
    # will keep subtracting one until it finds a cup that wasn't just picked up.
    while any([n.val == dst for n in pickup]):
        dst -= 1

        # If at any point in this process the value goes below the lowest value
        # on any cup's label, it wraps around to the highest value on any cup's
        # label instead.
        while dst <= 0:
            dst += len(cups)

    # The crab places the cups it just picked up so that they are immediately
    # clockwise of the destination cup. They keep the same order as when they
    # were picked up.
    cups[dst].next, pickup[-1].next = pickup[0], cups[dst].next

    # The crab selects a new current cup: the cup which is immediately
    # clockwise of the current cup.
    head = head.next
    # print()


# print(head.val, to_str_from_one())
print(f"{cups[1].next.val} * {cups[1].next.next.val} == {cups[1].next.val * cups[1].next.next.val}")
