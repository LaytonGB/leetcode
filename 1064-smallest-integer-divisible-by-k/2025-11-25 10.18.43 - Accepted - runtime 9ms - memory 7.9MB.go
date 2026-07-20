func smallestRepunitDivByK(k int) int {
    vis := make(map[int]struct{})
    n := 1
    i := 1
    for n % k != 0 {
        if _, visited := vis[n]; visited {
            return -1
        }
        vis[n] = struct{}{}

        n = (n * 10 + 1) % k
        
        i++
    }
    return i
}