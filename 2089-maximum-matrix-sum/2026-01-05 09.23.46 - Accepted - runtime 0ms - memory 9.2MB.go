func maxMatrixSum(matrix [][]int) int64 {
    negativeCount := 0
    smallestNegative := int(^uint(0) >> 1)
    var sum int64 = 0
    for _, row := range matrix {
        for _, val := range row {
            if val < 0 {
                negativeCount++
                smallestNegative = min(smallestNegative, -val)
                sum += int64(-val)
            } else {
                smallestNegative = min(smallestNegative, val)
                sum += int64(val)
            }
        }
    }
    
    if negativeCount % 2 == 1 {
        return sum - int64(smallestNegative) * 2
    }
    return sum
}