func longestBalanced(s string) int {
    res := 0
    for i := range len(s) {
        if len(s) - i <= res {
            break
        }
        
        counts := [26]int{}
        max_count := 0
        
        for j := range len(s) - i {
            c := s[i+j]
            counts[int(c - 'a')]++
            max_count = max(max_count, counts[int(c - 'a')])

            if is_balanced(counts, max_count) && j + 1 > res {
                res = j + 1
            }
        }
    }
    return res
}

func is_balanced(counts [26]int, max_count int) bool {
    for _, c := range counts {
        if c == 0 {
            continue
        }

        if c != max_count {
            return false
        }
    }
    return true
}
