func maximizeSquareHoleArea(n int, m int, hBars []int, vBars []int) int {
    slices.Sort(hBars)
    slices.Sort(vBars)

    maxHIncrease := 1
    maxVIncrease := 1
    increaseStreak := 1
    for i := range len(hBars) - 1 {
        if hBars[i] + 1 == hBars[i+1] {
            increaseStreak++
            if increaseStreak > maxHIncrease {
                maxHIncrease = increaseStreak
            }
        } else {
            increaseStreak = 1
        }
    }
    increaseStreak = 1
    for i := range len(vBars) - 1 {
        if vBars[i] + 1 == vBars[i+1] {
            increaseStreak++
            if increaseStreak > maxVIncrease {
                maxVIncrease = increaseStreak
                if maxVIncrease >= maxHIncrease {
                    return convertBarsRemovedToMaxSquare(maxHIncrease)
                }
            }
        } else {
            increaseStreak = 1
        }
    }

    return convertBarsRemovedToMaxSquare(maxVIncrease)
}

func convertBarsRemovedToMaxSquare(maxSharedBarsStreak int) int {
    side := maxSharedBarsStreak + 1
    fmt.Println(side)
    return side * side
}
