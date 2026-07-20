func hasAlternatingBits(n int) bool {
    bits := fmt.Sprintf("%b", n)
    one_seen := false
    last := '0'
    for _, b := range bits {
        if b == '1' {
            if last == '1' {
                return false
            }
            if one_seen == false {
                one_seen = true
            }
        } else if one_seen && b == '0' && last == '0' {
            return false
        }
        last = b
    }
    return true
}