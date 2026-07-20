func constructTransformedArray(nums []int) []int {
    n := len(nums)
    res := make([]int, n)
    for i := range n {
        idx := abs_mod(i + nums[i], n)
        res[i] = nums[idx]
    }
    return res
}

func abs_mod(val, n int) int {
    res := val % n
    for res < 0 {
        res += n
    }
    return res
}