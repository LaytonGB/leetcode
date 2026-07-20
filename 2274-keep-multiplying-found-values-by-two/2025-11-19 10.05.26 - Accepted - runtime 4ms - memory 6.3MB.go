func findFinalValue(nums []int, original int) int {
    set := make(map[int]struct{})
    for _, n := range nums {
        set[n] = struct{}{}
    }

    sortedNums := make([]int, len(set))
    i := 0
    for n := range set {
        sortedNums[i] = n
        i++
    }
    slices.Sort(sortedNums)

    startIndex := findIndex(sortedNums, original, 0)
    for startIndex != -1 {
        original *= 2
        startIndex = findIndex(sortedNums, original, startIndex)
    }

    return original
}

func findIndex(nums []int, target, startIndex int) int {
    l, h := startIndex, len(nums)
    for l < h {
        m := (l + h) / 2
        if nums[m] == target {
            return m
        } else if nums[m] < target {
            l = m + 1
        } else {
            h = m
        }
    }
    return -1
}