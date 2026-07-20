const mod int64 = 1_000_000_007

func specialTriplets(nums []int) int {
    prefix := make(map[int]int)
    suffix := make(map[int]int)

    // build suffix
    for _, x := range nums {
        if count, exists := suffix[x]; exists {
            suffix[x] = count + 1
        } else {
            suffix[x] = 1
        }
    }
    
    // shift values into prefix and accumulate result
    var res int64 = 0
    for _, x := range nums {
        suffix[x]--
        res += int64(prefix[x*2] * suffix[x*2])
        
        if count, exists := prefix[x]; exists {
            prefix[x] = count + 1
        } else {
            prefix[x] = 1
        }
    }

    return int(res % mod)
}
