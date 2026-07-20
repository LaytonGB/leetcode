func isPrefixOfWord(sentence string, searchWord string) int {
    arr := strings.Split(sentence, " ")
    if len(arr) == 0 {
        return 0
    }

    n := len(searchWord)
    for i, w := range arr {
        if len(w) >= n && w[:n] == searchWord {
            return i + 1
        }
    }

    return -1
}