type CoordMap map[[2]int]struct{}

func countCoveredBuildings(n int, builds [][]int) int {
    rows, cols := getOuter(builds)
    res := 0
    for _, coords := range builds {
        if isCovered(n, coords[0], coords[1], rows, cols) {
            res++
        }
    }
    return res
}

func getOuter(builds [][]int) (map[int][2]int, map[int][2]int) {
    byRow := make(map[int][2]int)
    byCol := make(map[int][2]int)
    for _, coords := range builds {
        row := coords[0]
        col := coords[1]
        
        if cols, exists := byRow[row]; exists {
            if col < cols[0] {
                byRow[row] = [2]int{col, cols[1]}
            } else if col > cols[1] {
                byRow[row] = [2]int{cols[0], col}
            }
        } else {
            byRow[row] = [2]int{col, col}
        }
        
        if rows, exists := byCol[col]; exists {
            if row < rows[0] {
                byCol[col] = [2]int{row, rows[1]}
            } else if row > rows[1] {
                byCol[col] = [2]int{rows[0], row}
            }
        } else {
            byCol[col] = [2]int{row, row}
        }
    }
    return byRow, byCol
}

func isCovered(n, r, c int, rows, cols map[int][2]int) bool {
    if r == 1 || r == n || c == 1 || c == n {
        return false
    }

    if c > rows[r][0] && c < rows[r][1] && r > cols[c][0] && r < cols[c][1] {
        return true
    }

    return false
}
