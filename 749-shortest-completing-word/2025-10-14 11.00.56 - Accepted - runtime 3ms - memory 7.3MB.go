var memo map[string]bool

func shortestCompletingWord(licensePlate string, words []string) string {
    plateCount := make(map[byte]int)
    for i := 0; i < len(licensePlate); i++ {
        b := licensePlate[i]

        // Convert lowercase to uppercase
        if b >= 97 && b <= 122 {
            b -= 32
        }

        // Ignore non-letter
        if b < 65 || b > 90 {
            continue
        }

        if n, ok := plateCount[b]; ok {
            plateCount[b] = n + 1
        } else {
            plateCount[b] = 1
        }
    }

    shortestLen := int(^uint(0) >> 1)
    var res string
    for _, w := range words {
        if len(w) >= shortestLen || !doesContainAll(plateCount, w) {
            continue
        }

        res = w
        shortestLen = len(w)
    }

    return res
}

func doesContainAll(target map[byte]int, str string) bool {
    tgt := maps.Clone(target)
    s := strings.ToUpper(str)
    for i := 0; i < len(s); i++ {
        b := s[i]
        if n, ok := tgt[b]; ok {
            if n == 1 {
                delete(tgt, b)
            } else {
                tgt[b] = n - 1
            }
        }

        if len(tgt) == 0 {
            // fmt.Printf("target %v\ntgt %v\ns %v\ncontained all", target, tgt, s)
            return true
        }
    }
    // fmt.Printf("target %v\ntgt %v\ns %v\ndid not contain all", target, tgt, s)
    return false
}
