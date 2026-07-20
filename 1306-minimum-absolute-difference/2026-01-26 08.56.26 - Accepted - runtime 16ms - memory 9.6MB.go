func minimumAbsDifference(arr []int) [][]int {
    var (
        lowest_pairs [][]int
        lowest int = int(^uint(0) >> 1)
    )
    slices.Sort(arr)
    for i := range len(arr) - 1 {
        pair_diff := arr[i+1] - arr[i]
        if pair_diff < lowest {
            lowest = pair_diff
            lowest_pairs = [][]int{[]int{arr[i], arr[i+1]}}
        } else if pair_diff == lowest {
            lowest_pairs = append(lowest_pairs, []int{arr[i], arr[i+1]})
        }
    }
    return lowest_pairs
}