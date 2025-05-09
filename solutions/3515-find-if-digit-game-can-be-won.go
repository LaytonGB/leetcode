func canAliceWin(nums []int) bool {
    diff := 0
    for _, x := range nums {
        if x < 10 {
            diff += x
        } else {
            diff -= x
        }
    }
    return diff != 0
}
