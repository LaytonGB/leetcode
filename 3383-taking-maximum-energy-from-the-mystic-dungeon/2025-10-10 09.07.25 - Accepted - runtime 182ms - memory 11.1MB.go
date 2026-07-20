func maximumEnergy(energy []int, k int) int {
    n := len(energy)
    res := energy[len(energy)-1]
    for i := n - 1; i >= 0; i-- {
        if i + k < n {
            energy[i] += energy[i+k]
        }
        res = max(res, energy[i])
    }
    return res
}