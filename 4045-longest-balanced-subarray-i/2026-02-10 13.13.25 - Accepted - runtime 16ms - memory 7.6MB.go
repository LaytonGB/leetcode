// https://leetcode.com/problems/longest-balanced-subarray-i/solutions/7567016/lazy-reset-no-hash-set-by-la_castille-7v3h

func longestBalanced(nums []int) int {
    n := len(nums)
    res := 0
    seen := [100001]int{}

    for i := range n {
        // break if potential gain is less than current best
        if n - i <= res {
            break
        }

        counts := [2]int{0, 0}
        for j := range n - i {
            val := nums[i+j]
            if seen[val] != i + 1 {
                seen[val] = i + 1
                counts[val & 1]++
            }

            if counts[0] == counts[1] {
                res = max(res, j + 1)
            }
        }
    }

    return res
}