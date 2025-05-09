type Dir int

const (
    L Dir = iota
    R
    U
    D
)

func slidingPuzzle(board [][]int) int {
    b := sliceToArray(board)
    s := make(map[[2][3]int]struct{})
    q1 := make([]([2][3]int), 1)
    q2 := make([]([2][3]int), 0)
    q1[0] = b

    count := 0
    for len(q1) != 0 {
        q1, q2 = q2, q1
        q1 = q1[:0]
        for _, board := range q2 {
            if _, ok := s[board]; ok {
                continue;
            }
            s[board] = struct{}{}

            if getIsCorrect(board) {
                return count
            }
            
            zRow, zCol := getZeroPos(board)
            for _, dir := range []Dir{L, R, U, D} {
                swapped, ok := tryGetSwapped(board, zRow, zCol, dir)
                if ok {
                    q1 = append(q1, swapped)
                }
            }
        }
        count += 1
    }

    return -1
}

func sliceToArray(board [][]int) [2][3]int {
    var res [2][3]int
    for row := range 2 {
        for col := range 3 {
            res[row][col] = board[row][col]
        }
    }
    return res
}

func copyArray(arr [2][3]int) [2][3]int {
    var res [2][3]int
    for row := range 2 {
        for col := range 3 {
            res[row][col] = arr[row][col]
        }
    }
    return res
}

func getZeroPos(board [2][3]int) (int, int) {
    for i := range 2 {
        for j := range 3 {
            if board[i][j] == 0 {
                return i, j
            }
        }
    }
    return -1, -1
}

func getAdjPos(posRow int, posCol int, dir Dir) (int, int) {
    switch dir {
        case L:
            return posRow, posCol - 1
        case R:
            return posRow, posCol + 1
        case U:
            return posRow - 1, posCol
        case D:
            return posRow + 1, posCol
        default:
            return -1, -1
    }
}

func swap(board [2][3]int, zeroRow int, zeroCol int, posRow int, posCol int) [2][3]int {
    board[zeroRow][zeroCol], board[posRow][posCol] =
        board[posRow][posCol], board[zeroRow][zeroCol]
    return board
}

func tryGetSwapped(board [2][3]int, zRow int, zCol int, dir Dir) ([2][3]int, bool) {
    b := copyArray(board)
    r, c := getAdjPos(zRow, zCol, dir)
    if r < 0 || r > 1 || c < 0 || c > 2 {
        return b, false
    }
    b = swap(b, zRow, zCol, r, c)
    return b, true
}

func getIsCorrect(board [2][3]int) bool {
    return board[0][0] == 1 && board[0][1] == 2 &&
        board[0][2] == 3 && board[1][0] == 4 &&
        board[1][1] == 5 && board[1][2] == 0
}
