func numberOfBeams(bank []string) int {
    crossSum := 0
    prevLasers := 0
    for _, r := range bank {
        rowLasers := 0
        for i := 0; i < len(r); i++ {
            if r[i] == '1' {
                rowLasers++
            }
        }
        if rowLasers == 0 {
            continue
        }
        crossSum += prevLasers * rowLasers
        prevLasers = rowLasers
    }
    return crossSum
}