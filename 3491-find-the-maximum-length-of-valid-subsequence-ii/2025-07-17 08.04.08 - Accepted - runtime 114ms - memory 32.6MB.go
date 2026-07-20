func maximumLength(nums []int, k int) int {
    dp := make([][]int, k)
    for i := range dp {
        dp[i] = make([]int, k)
    }

    res := 0
    for i := range nums {
        for j := range k {
            newDp := dp[j][nums[i] % k] + 1
            dp[nums[i] % k][j] = newDp
            res = max(res, newDp)
        }
    }

    return res
}