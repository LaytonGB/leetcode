func minBitwiseArray(nums []int) []int {
    res := make([]int, len(nums))
    for i := range len(nums) {
        if nums[i] % 2 == 0 {
            res[i] = -1
        } else {
            res[i] = nums[i] & ^(((nums[i] + 1) & ^nums[i]) >> 1)
        }
    }
    return res
}