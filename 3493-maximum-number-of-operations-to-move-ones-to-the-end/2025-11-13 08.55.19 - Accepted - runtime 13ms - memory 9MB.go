func maxOperations(s string) int {
    var res, ones int
    parts := strings.Split(s, "0")
    n := len(parts)
    for i := range n - 1 {
        if len(parts[i]) > 0 {
            res += ones + len(parts[i])
            ones += len(parts[i])
        }
    }
    return res
}