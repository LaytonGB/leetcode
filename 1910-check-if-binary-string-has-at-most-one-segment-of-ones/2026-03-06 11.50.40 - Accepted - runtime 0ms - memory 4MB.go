func checkOnesSegment(s string) bool {
    var last uint8 = '1'
    for i := range len(s) - 1 {
        if s[i+1] == '1' && last != '1' {
            return false
        }
        last = s[i+1]
    }
    return true
}