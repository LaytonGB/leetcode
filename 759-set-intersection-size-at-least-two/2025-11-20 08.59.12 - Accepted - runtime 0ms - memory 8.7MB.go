func intersectionSizeTwo(intervals [][]int) int {
    slices.SortFunc(intervals, sortBAscThenADesc)

    count := 2
    n := len(intervals)
    b := intervals[0][1]
    a := b - 1
    for i := 1; i < n; i++ {
        l, r := intervals[i][0], intervals[i][1]
        if a >= l {
            continue
        }

        no := l > b
        if no {
            count += 2
            a = r - 1
        } else {
            count += 1
            a = b
        }
        b = r
    }
    return count
}

func sortBAscThenADesc(a, b []int) int {
    if a[1] == b[1] {
        return b[0] - a[0]
    }
    return a[1] - b[1]
}
