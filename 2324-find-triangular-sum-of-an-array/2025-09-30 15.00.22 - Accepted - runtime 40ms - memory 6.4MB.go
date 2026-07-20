func triangularSum(nums []int) int {
    if len(nums) == 1 {
        return nums[0]
    }
    
    return h(nums, len(nums) - 1)
}

func h(nums []int, n int) int {
    if n == 1 {
        return (nums[0] + nums[1]) % 10
    }

    for i := range n {
        nums[i] = (nums[i] + nums[i+1]) % 10
    }
    
    return h(nums, n - 1)
}