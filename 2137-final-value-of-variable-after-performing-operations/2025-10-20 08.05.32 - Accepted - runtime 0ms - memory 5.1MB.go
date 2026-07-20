func finalValueAfterOperations(operations []string) int {
    res := 0
    for _, o := range operations {
        if o == "++X" || o == "X++" {
            res++
        } else {
            res--
        }
    }
    return res
}