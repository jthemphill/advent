import sys

def threesum(nums):
    nums.sort()
    for i, a in enumerate(nums):
        start = i + 1
        end = len(nums) - 1
        while start < end:
            b = nums[start]
            c = nums[end]
            if a + b + c == 2020:
                return (a, b, c)
            elif a + b + c < 2020:
                start += 1
            else:
                end -= 1

nums = []
for line in sys.stdin:
    nums.append(int(line))

(a, b, c) = threesum(nums)
print(f"{a} * {b} * {c} = {a * b * c}")