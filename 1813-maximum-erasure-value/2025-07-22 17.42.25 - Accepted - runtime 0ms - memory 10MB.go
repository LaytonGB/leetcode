func maximumUniqueSubarray(nums []int) int {
    var (
        a, b, n, setSum, res     int
        set                         [10001]bool
    )
    for ; b < len(nums); b++ {
        n = nums[b]

        if set[n] {
            for nums[a] != n {
                set[nums[a]] = false
                setSum -= nums[a]
                a++
            }
            a++
        } else {
            set[n] = true
            setSum += n
        }

        res = max(res, setSum)
    }

    return res
}
