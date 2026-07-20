func maxIncreasingSubarrays(nums []int) int {
    var streaks []int
    s := 0
    highestIndividualStreak := 0
    for i := range len(nums) - 1 {
        if s > highestIndividualStreak {
            highestIndividualStreak = s
        }
        if nums[i] < nums[i+1] {
            s += 1
        } else {
            streaks = append(streaks, s)
            s = 0
        }
    }
    if s > 0 {
        if s > highestIndividualStreak {
            highestIndividualStreak = s
        }
        streaks = append(streaks, s)
    }

    highestLowFromPairs := 0
    for i := range len(streaks) - 1 {
        b := min(streaks[i], streaks[i+1])
        if b > highestLowFromPairs {
            highestLowFromPairs = b
        }
    }

    return max(highestLowFromPairs + 1, (highestIndividualStreak + 1) / 2)
}