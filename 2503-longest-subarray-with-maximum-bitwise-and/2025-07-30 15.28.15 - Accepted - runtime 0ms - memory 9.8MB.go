func longestSubarray(nums []int) int {
    var (
        maxN, curr, res int
    )
    
    for _, n := range nums {
        maxN = max(maxN, n)
    }
    
    for _, n := range nums {
        if n == maxN {
            curr += 1
            res = max(res, curr)
        } else {
            curr = 0
        }
    }

    return res
}