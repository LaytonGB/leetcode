func countBinarySubstrings(s string) int {
    consecutiveCounts := [100000]int{}
    idx := 0
    state := s[0]
    count := 0
    for i := range len(s) {
        if s[i] == state {
            count++
        } else {
            consecutiveCounts[idx] = count
            state = s[i]
            count = 1
            idx++
        }
    }
    consecutiveCounts[idx] = count

    res := 0
    for i := range idx {
        res += min(consecutiveCounts[i], consecutiveCounts[i+1])
    }

    return res
}