var (
    m, dp [][]int
    coords = [][]int{
        []int{0, -1},
        []int{-1, 0},
        []int{-1, -1},
    }
)

func countSquares(matrix [][]int) int {
    m = matrix

    // Duplicate m size
    dp = make([][]int, len(m))
    for i := range m {
        dp[i] = make([]int, len(m[i]))
    }

    // Populate all values in dp
    for i := range m {
        for j := range m[i] {
            getSquareSizeFromBottomRight(i, j)
        }
    }

    // Get sum of all dp values
    dpSum := 0
    for _, row := range dp {
        for _, val := range row {
            dpSum += val
        }
    }

    fmt.Println(dp)
    return dpSum
}

func getSquareSizeFromBottomRight(i, j int) int {
    if m[i][j] == 0 {
        return 0;
    }

    if dp[i][j] != 0 {
        return dp[i][j]
    }

    if i == 0 || j == 0 {
        dp[i][j] = 1
        return 1
    }

    minOfTopLeftValues := 300 // max grid size
    for _, adj := range coords {
        minOfTopLeftValues = min(
            minOfTopLeftValues,
            getSquareSizeFromBottomRight(i + adj[0], j + adj[1]))
    }

    dp[i][j] = minOfTopLeftValues + 1
    return minOfTopLeftValues + 1
}