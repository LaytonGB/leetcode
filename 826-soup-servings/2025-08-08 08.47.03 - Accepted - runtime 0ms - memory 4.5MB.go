type Key = struct {
    a float64
    b float64
}

var (
    memo map[Key]float64
)

func init() {
    memo = make(map[Key]float64)
}

func soupServings(n int) float64 {
    if n > 4800 {
        return 1
    }

    m := math.Ceil(float64(n) / 25.0)
    return f(m, m)
}

func f(a, b float64) float64 {
    if val, exists := memo[Key{a, b}]; exists {
        return val;
    }

    if a <= 0 && b <= 0 {
        return 0.5
    }
    if a <= 0 {
        return 1
    }
    if b <= 0 {
        return 0
    }

    m := 0.25 * (f(a-4, b) + f(a-3, b-1) + f(a-2, b-2) + f(a-1, b-3))
    memo[Key{a, b}] = m

    return m
}