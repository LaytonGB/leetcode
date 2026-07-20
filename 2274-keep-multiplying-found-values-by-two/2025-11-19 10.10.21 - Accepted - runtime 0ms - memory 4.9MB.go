func findFinalValue(nums []int, original int) int {
    slices.Sort(nums)

    startIndex := findIndex(nums, original, 0)
    for startIndex != -1 {
        original *= 2
        startIndex = findIndex(nums, original, startIndex)
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