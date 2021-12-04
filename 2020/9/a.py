import sys


def firstinvalid(nums, npreamble):
    for i in range(0, len(nums) - npreamble):
        preamble = set(nums[i:i+npreamble])
        twosum = nums[i+npreamble]
        if not any([twosum - n in preamble for n in preamble]):
            return twosum
    return None


def findcontiguous(nums, target):
    total = nums[0]
    head = 0
    tail = 1
    while tail < len(nums):
        assert head <= tail
        assert sum(nums[head:tail]) == total
        if total == target:
            return nums[head:tail]
        elif total > target:
            total -= nums[head]
            head += 1
        else:
            assert total < target
            total += nums[tail]
            tail += 1
    return None


nums = []
for line in sys.stdin:
    nums.append(int(line.rstrip()))
firstinvalid = firstinvalid(nums, 25)
print(f"Twosum answer: {firstinvalid}")
contiguous = findcontiguous(nums, firstinvalid)
print(f"sum({contiguous}) = {firstinvalid}")
assert sum(contiguous) == firstinvalid
lo = min(contiguous)
hi = max(contiguous)
print(f"{lo} + {hi} = {lo + hi}")