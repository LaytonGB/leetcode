class Solution:
    def minimumAverage(self, nums: List[int]) -> float:
        nums.sort()
        return min((nums[i] + nums[-i - 1] for i in range(len(nums) // 2))) / 2
