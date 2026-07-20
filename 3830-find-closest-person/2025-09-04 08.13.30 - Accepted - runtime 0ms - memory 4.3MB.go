func findClosest(x int, y int, z int) int {
    xDiff := int(math.Abs(float64(x - z)))
    yDiff := int(math.Abs(float64(y - z)))

    if xDiff < yDiff {
        return 1
    } else if xDiff > yDiff {
        return 2
    } else {
        return 0
    }
}