func numberOfBeams(bank []string) int {
    crossSum := 0
    prevLasers := 0
    for _, r := range bank {
        rowLasers := strings.Count(r, "1")
        if rowLasers > 0 {
            crossSum += prevLasers * rowLasers
            prevLasers = rowLasers
        }
    }
    return crossSum
}