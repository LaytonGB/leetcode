func minPartitions(n string) int {
    maxInt := 0
    for _, c := range n {
        x := int(c - '0')
        maxInt = max(maxInt, x)
    }
    return maxInt
}