var vowels = []rune{'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'}

func sortVowels(s string) string {
    vowelIndices := make([]int, 0)
    v := make([]rune, 0)

    r := []rune(s)

    for index, char := range r {
        if slices.Contains(vowels, char) {
            vowelIndices = append(vowelIndices, index)
            v = append(v, char)
        }
    }

    slices.Sort(v)

    for index := range len(vowelIndices) {
        sIndex := vowelIndices[index]
        r[sIndex] = v[index]
    }

    return string(r)
}