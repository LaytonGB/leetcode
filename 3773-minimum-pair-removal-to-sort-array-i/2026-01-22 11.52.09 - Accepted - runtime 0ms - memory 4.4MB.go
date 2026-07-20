func minimumPairRemoval(nums []int) int {
    res := 0
    for !isSorted(nums) {
        minPairIdx := -1
        minPairSum := int(^uint(0) >> 1) // init with min-value

        for i := range len(nums) - 1 {
            if nums[i] + nums[i+1] < minPairSum {
                minPairIdx = i
                minPairSum = nums[i] + nums[i+1]
            }
        }

        nums = append(
            append(nums[:minPairIdx], nums[minPairIdx] + nums[minPairIdx+1]),
            nums[minPairIdx+2:]...,
        )

        res++
    }
    return res
}

func isSorted(s []int) bool {
    for i := range len(s) - 1 {
        if s[i] > s[i+1] {
            return false
        }
    }
    return true
}
