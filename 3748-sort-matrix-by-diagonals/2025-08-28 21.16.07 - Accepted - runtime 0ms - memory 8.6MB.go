func sortMatrix(grid [][]int) [][]int {
    n := len(grid)
    
    lowerDiagonals := n
    for i := range lowerDiagonals {
        diagonalValues := getDiagonalValues(grid, n, i)
        slices.SortFunc(diagonalValues, func (a, b int) int { return b - a })
        writeDiagonalValues(grid, n, i, diagonalValues)
    }

    upperDiagonals := n - 1
    for i := range upperDiagonals {
        i := i + n
        diagonalValues := getDiagonalValues(grid, n, i)
        slices.Sort(diagonalValues)
        writeDiagonalValues(grid, n, i, diagonalValues)
    }
    
    return grid
}

func getDiagonalStartPosition(n, diagonalIndex int) (int, int, int) {
    r := max(0, n - diagonalIndex - 1)
    c := max(0, diagonalIndex - n + 1)
    diagonalLen := n - max(r, c)

    return r, c, diagonalLen
}

func getDiagonalValues(grid [][]int, n, diagonalIndex int) []int {
    r, c, diagonalLen := getDiagonalStartPosition(n, diagonalIndex)
    
    res := make([]int, diagonalLen)
    for i := range diagonalLen {
        res[i] = grid[r][c]
        r += 1
        c += 1
    }

    return res
}

func writeDiagonalValues(grid [][]int, n, diagonalIndex int, valuesToInsert []int) {
    r, c, diagonalLen := getDiagonalStartPosition(n, diagonalIndex)
    
    for i := range diagonalLen {
        grid[r][c] = valuesToInsert[i]
        r += 1
        c += 1
    }
}
