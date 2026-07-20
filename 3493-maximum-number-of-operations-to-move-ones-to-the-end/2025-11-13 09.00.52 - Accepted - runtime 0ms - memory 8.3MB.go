func maxOperations(s string) int {
    var res, ones int
    n := len(s)
    for i := range n {
        if s[i] == '1' {
            ones += 1
        } else if i > 0 && s[i-1] == '1' {
            res += ones
        }
    }
    return res
}