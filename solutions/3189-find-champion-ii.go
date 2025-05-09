func findChampion(n int, edges [][]int) int {
    g := make(map[int][]int)
    for _, e := range edges {
        v, ok := g[e[1]]
        if ok {
            g[e[1]] = append(v, e[0])
        } else {
            g[e[1]] = []int{e[0]}
        }
    }

    champs := make(map[int]struct{})
    for i := range n {
        champs[i] = struct{}{}
    }

    for i := range n {
        if _, ok := g[i]; ok {
            delete(champs, i)
        }
    }

    if len(champs) == 1 {
        for k := range champs {
            return k
        }
    }
    
    return -1
}