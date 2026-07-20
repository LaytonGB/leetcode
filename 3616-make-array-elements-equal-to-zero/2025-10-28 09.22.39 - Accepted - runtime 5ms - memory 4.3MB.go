func countValidSelections(nums []int) int {
    n := len(nums)
    lSum := make([]int, n)
    lSum[0] = nums[0]
    rSum := make([]int, n)
    rSum[n-1] = nums[n-1]
    for i := range n - 1 {
        lSum[i+1] = lSum[i] + nums[i+1]
        rSum[n-i-2] = rSum[n-i-1] + nums[n-i-2]
    }

    // fmt.Printf("lSum: %v\nrSum: %v\n", lSum, rSum)

    matchingCount := 0
    for i := range n {
        diff := int(math.Abs(float64(lSum[i] - rSum[i])))
        // fmt.Printf("diff: %v\n", diff)
        if diff <= 1 && nums[i] == 0 {
            if diff == 1 {
                matchingCount += 1
            } else {
                matchingCount += 2
            }
        }
    }

    return matchingCount
}