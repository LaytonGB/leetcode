func subarrayBitwiseORs(arr []int) int {
    res, cur, cur2 := make(map[int]struct{}), make(map[int]struct{}), make(map[int]struct{})

    for _, a := range arr {
        clear(cur2)
        cur2[a] = struct{}{}

        for b := range cur {
            cur2[a | b] = struct{}{}
        }

        cur = maps.Clone(cur2)
        maps.Copy(res, cur)
    }

    return len(res)
}