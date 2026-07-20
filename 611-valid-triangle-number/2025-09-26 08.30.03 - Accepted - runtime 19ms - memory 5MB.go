func triangleNumber(nums []int) int {
    slices.Sort(nums)

    res := 0
    for i := len(nums) - 1; i >= 0; i-- {
        left, right := 0, i - 1
        for left < right {
            if nums[left] + nums[right] > nums[i] {
                res += right - left
                right--
            } else {
                left++
            }
        }
    }

    return res
}
