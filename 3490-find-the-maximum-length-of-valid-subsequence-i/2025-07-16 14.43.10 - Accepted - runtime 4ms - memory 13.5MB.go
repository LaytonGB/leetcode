func maximumLength(nums []int) int {
    y := nums[0] % 2
    ev, odd, neq := 0, 0, 0
    for _, x := range nums {
        if x % 2 == 0 {
            ev += 1
        } else {
            odd += 1
        }
        
        if x % 2 == y {
            neq += 1
            y ^= 1
        }
    }

    return max(ev, odd, neq)
}