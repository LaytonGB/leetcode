func maximumLength(nums []int, k int) int {
    dp := make([][]int, k)
    for i := range dp {
        dp[i] = make([]int, k)
    }

    res := 0
    for i := range nums {
        r := nums[i] % k
        for j := range k {
            newDp := dp[j][r] + 1
            dp[r][j] = newDp
            res = max(res, newDp)
        }
    }

    return res
}