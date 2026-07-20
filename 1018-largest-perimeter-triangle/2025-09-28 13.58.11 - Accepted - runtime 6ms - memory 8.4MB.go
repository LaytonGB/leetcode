func largestPerimeter(nums []int) int {
    n := len(nums)
    slices.Sort(nums)

    i := n
    for i >= 3 && nums[i-1] >= nums[i-2] + nums[i-3] {
        i -= 1
    }

    if i < 3 || nums[i-1] >= nums[i-2] + nums[i-3] {
        return 0
    }

    return nums[i-1] + nums[i-2] + nums[i-3]
}