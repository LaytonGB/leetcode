from itertools import permutations as perm

class Solution:
    def maxSumRangeQuery(self, nums: List[int], requests: List[List[int]]) -> int:
        pos_count = [0] * len(nums)
        for l,h in requests:
            pos_count[l] += 1
            if h+1 < len(pos_count):
                pos_count[h+1] -= 1
        for i in range(1, len(pos_count)):
            pos_count[i] += pos_count[i-1]
        pos_count.sort()
        nums.sort()
        return sum(a * b for a,b in zip(pos_count, nums)) % (10**9 + 7)
        
        