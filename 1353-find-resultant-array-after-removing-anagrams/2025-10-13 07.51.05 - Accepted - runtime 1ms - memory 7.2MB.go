type Count [26]int

type MaybeCount struct {
    IsSome bool
    Count Count
}

var memo []MaybeCount

func removeAnagrams(words []string) []string {
    memo = make([]MaybeCount, len(words))
    for i := len(words) - 1; i > 0; i-- {
        if getCount(words, i - 1) == getCount(words, i) {
            words = append(words[:i], words[i+1:]...)
        }
    }
    return words
}

func getCount(words []string, i int) Count {
    if memo[i].IsSome {
        return memo[i].Count
    }

    var c Count
    w := words[i]
    for j := 0; j < len(w); j++ {
        c[w[j]-'a'] += 1
    }

    memo[i] = MaybeCount{IsSome: true, Count: c}
    
    return c
}