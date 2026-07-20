func maximumUniqueSubarray(nums []int) int {
    res := 0
    set := make(map[int]int)
    setSum := 0
    a := 0
    for b := 0; b < len(nums); b++ {
        addedNum := nums[b]

        if lastOccurenceIndex, isValueInSet := set[addedNum]; isValueInSet {
            for a <= lastOccurenceIndex {
                delete(set, nums[a])
                setSum -= nums[a]
                a++
            }
        }
        set[addedNum] = b
        setSum += addedNum

        res = max(res, setSum)
    }

    return res
}
