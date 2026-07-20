func hasIncreasingSubarrays(nums []int, k int) bool {
    streaks := []int{0}
    currStreak := 0
    for i := range len(nums) - 1 {
        if nums[i] < nums[i+1] {
            currStreak += 1
        } else {
            if isValidStreak(streaks, currStreak, k) {
                return true
            }
            
            streaks = append(streaks, currStreak)
            currStreak = 0
        }
    }

    if isValidStreak(streaks, currStreak, k) {
        return true
    }

    return false
}

func isValidStreak(streaks []int, currStreak, k int) bool {
    return (currStreak >= k - 1 && streaks[len(streaks)-1] >= k - 1) || currStreak >= k * 2 - 1
}
