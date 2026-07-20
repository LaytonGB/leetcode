func prefixCount(words []string, pref string) int {
    sum := 0
    for _, w := range words {
        if strings.HasPrefix(w, pref) {
            sum += 1
        }
    }
    return sum
}