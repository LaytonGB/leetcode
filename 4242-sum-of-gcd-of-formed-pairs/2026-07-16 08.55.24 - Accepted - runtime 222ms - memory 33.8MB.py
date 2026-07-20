import math

class Solution:
    def gcdSum(self, nums: list[int]) -> int:
        limMax = 0
        prefixGcd = [0] * len(nums)
        for i in range(len(nums)):
            limMax = max(limMax, nums[i])
            prefixGcd[i] = math.gcd(limMax, nums[i])
        
        prefixGcd = sorted(prefixGcd)
        
        n = len(prefixGcd)
        lim = n // 2
        i = 0
        res = 0
        while i < lim:
            res += math.gcd(prefixGcd[i], prefixGcd[n - i - 1])
            i += 1
        
        return res