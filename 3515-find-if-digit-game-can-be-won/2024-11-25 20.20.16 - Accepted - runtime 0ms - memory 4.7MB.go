func canAliceWin(nums []int) bool {
    var singles, doubles int
    for _, x := range nums {
        if x < 10 {
            singles += x
        } else {
            doubles += x
        }
    }
    return singles != doubles
}
