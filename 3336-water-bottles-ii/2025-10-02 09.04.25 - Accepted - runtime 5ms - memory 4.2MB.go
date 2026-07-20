func maxBottlesDrunk(numBottles int, numExchange int) int {
    res := 0
    for {
        if numBottles >= numExchange {
            numBottles -= numExchange - 1
            res += numExchange
            numExchange += 1
        } else {
            res += numBottles
            break
        }
    }
    return res
}