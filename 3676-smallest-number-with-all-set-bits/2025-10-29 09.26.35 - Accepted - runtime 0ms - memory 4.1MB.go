func smallestNumber(n int) int {
    res := 0
    for x := 1; x <= n; x *= 2 {
        res += x
    }
    return res
}