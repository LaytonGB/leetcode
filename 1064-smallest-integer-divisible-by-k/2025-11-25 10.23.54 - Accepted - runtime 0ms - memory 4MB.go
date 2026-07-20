func smallestRepunitDivByK(k int) int {
    n := 0
    for i := 1; i <= k; i++ {
        n = (n * 10 + 1) % k
        if n == 0 { return i }
    }
    return -1
}