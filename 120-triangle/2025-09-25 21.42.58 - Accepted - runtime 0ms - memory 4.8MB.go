func minimumTotal(triangle [][]int) int {
    // Start bottom row,
    // add to options going upward
    // already contains current value
    // take top value

    for i := len(triangle) - 2; i >= 0; i-- {
        for j, t := range triangle[i] {
            triangle[i][j] = t + min(
                triangle[i + 1][j],
                triangle[i + 1][j + 1])
        }
    }

    return triangle[0][0]
}
