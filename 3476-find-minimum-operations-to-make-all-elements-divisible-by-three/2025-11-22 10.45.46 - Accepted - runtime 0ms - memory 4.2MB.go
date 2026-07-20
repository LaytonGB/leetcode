func minimumOperations(nums []int) int {
    res := 0
    for _, n := range nums {
        if n % 3 != 0 {
            res += 1
        }
    }
    return res
}