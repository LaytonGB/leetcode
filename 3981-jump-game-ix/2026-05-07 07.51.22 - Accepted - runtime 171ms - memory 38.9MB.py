def canJumpLeft(nums, i, j):
    return i > j and nums[i] < nums[j]


def canJumpRight(nums, i, j):
    return i < j and nums[i] > nums[j]


def getPreMax(nums):
    preMax = [0] * len(nums)
    preMax[0] = nums[0]
    for i in range(1, len(nums)):
        preMax[i] = max(preMax[i-1], nums[i])
    return preMax


class Solution:
    def maxValue(self, nums: List[int]) -> List[int]:
        preMax = getPreMax(nums)
        res = [0] * len(nums)
        sufMin = float('inf')
        for i in range(len(nums)-1, -1, -1):
            if preMax[i] > sufMin:
                res[i] = res[i+1]
            else:
                res[i] = preMax[i]

            sufMin = min(nums[i], sufMin)

        return res

        
