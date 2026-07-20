func numWaterBottles(numBottles int, numExchange int) int {
    res := 0
    empty := 0
    for numBottles > 0 {
        res += numBottles
        empty += numBottles
        emptyTradedIn := empty / numExchange * numExchange
        numBottles = empty / numExchange
        empty -= emptyTradedIn
    }
    return res
}