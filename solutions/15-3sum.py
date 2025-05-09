from itertools import combinations as C

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        out = []
        nums.sort()
        for i in range(len(nums)-2):
            if i > 0 and nums[i] == nums[i-1]:
                continue
            l = i + 1
            r = len(nums) - 1
            while l < r:
                t = (nums[i], nums[l], nums[r])
                s = sum(t)
                if s < 0:
                    l += 1
                elif s > 0:
                    r -= 1
                else:
                    out.append(t)
                    while l < r and nums[l] == nums[l+1]:
                        l += 1
                    while l < r and nums[r] == nums[r-1]:
                        r -= 1
                    l += 1; r -= 1
        return out