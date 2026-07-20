func prefixesDivBy5(nums []int) []bool {
    res := make([]bool, len(nums))
    n := 0
    for i, x := range nums {
        n = (n << 1 | x) % 5
        res[i] = n == 0 
    }
    return res
}