type set = map[int]struct{}
type pointList = []int
type pointMap = map[int]float64

func separateSquares(squares [][]int) float64 {
    var (
        target = getTotalArea(squares) / 2
        points, rates = getPointsAndRateChangeMap(squares)
        yPrev, yNext = 0, 0
        ratePrev, rateNext float64 = 0, 0
        totalPrev, totalNext float64 = 0, 0
    )
    
    i := 0
    for totalNext < target {
        yPrev, ratePrev, totalPrev = yNext, rateNext, totalNext

        yNext = points[i]
        rateNext += rates[yNext]
        totalNext += ratePrev * float64(yNext - yPrev)

        i++

        // fmt.Printf("target:%v\npoints:%v\nrates:%v\nyPrev:%v\nyNext:%v\nratePrev:%v\nrateNext:%v\ntotalPrev:%v\ntotalNext:%v\n\n",
        //     target, points, rates, yPrev, yNext, ratePrev, rateNext, totalPrev, totalNext)
    }

    dt := target - totalPrev
    dy := dt / ratePrev
    return float64(yPrev) + dy
}

func getTotalArea(squares [][]int) float64 {
    var res float64 = 0
    for _, square := range squares {
        res += float64(square[2] * square[2])
    }
    return res
}

func getPointsAndRateChangeMap(squares [][]int) (pointList, pointMap) {
    pointsSet := make(set)
    rates := make(pointMap)
    for _, square := range squares {
        pointsSet[square[1]] = struct{}{}
        pointsSet[square[1]+square[2]] = struct{}{}

        rates[square[1]] += float64(square[2])
        rates[square[1]+square[2]] -= float64(square[2])
    }
    points := pointList{}
    for p := range pointsSet {
        points = append(points, p)
    }
    slices.Sort(points)
    return points, rates
}
