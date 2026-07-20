func minTimeToVisitAllPoints(points [][]int) int {
    res := 0
    for i := range len(points) - 1 {
        a := [2]int{points[i][0], points[i][1]}
        b := [2]int{points[i+1][0], points[i+1][1]}
        res += getTimeBetweenPoints(a, b)
    }
    return res
}

func getTimeBetweenPoints(a, b [2]int) int {
    dx := abs(a[0] - b[0])
    dy := abs(a[1] - b[1])
    return max(dx, dy)
}

func abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
}
