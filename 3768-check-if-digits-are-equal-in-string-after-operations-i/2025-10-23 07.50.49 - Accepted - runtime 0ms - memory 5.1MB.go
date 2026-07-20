const OFFSET byte = '0' * 2

func hasSameDigits(s string) bool {
    var b strings.Builder
    for len(s) > 2 {
        for i := range len(s) - 1 {
            x := (s[i] + s[i+1] - OFFSET) % 10 + '0'
            b.WriteByte(x)
        }
        s = b.String()
        b.Reset()
    }
    return s[0] == s[1]
}