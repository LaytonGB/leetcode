func generate(numRows int) [][]int {
    res := make([][]int, numRows)
    res[0] = []int{1}

    if numRows >= 2 {
        res[1] = []int{1, 1}
    }

    for i := range max(0, numRows - 2) {
        row := make([]int, i + 3)

        row[0] = 1
        row[len(row) - 1] = 1
        for j := range len(row) - 2 {
            row[j + 1] = res[i + 1][j] + res[i + 1][j + 1]
        }

        res[i + 2] = row
    }

    return res
}