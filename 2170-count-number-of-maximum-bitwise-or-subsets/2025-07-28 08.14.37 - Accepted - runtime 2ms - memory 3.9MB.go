// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/solutions/5928761/beats-96-00-step-by-step-breakdown
func countMaxOrSubsets(nums []int) int {
    maxOr := 0
    for _, n := range nums {
        maxOr |= n
    }

    count := 0
    r(nums, 0, 0, maxOr, &count)

    return count
}

func r(nums []int, index, currentOr, maxOr int, count *int) {
    if currentOr == maxOr {
        *count++
    }

    for i := index; i < len(nums); i++ {
        r(nums, i + 1, currentOr | nums[i], maxOr, count)
    }
}