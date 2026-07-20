func isValid(word string) bool {
    if len(word) < 3 {
        return false
    }
    
    vowels := []rune("aeiouAEIOU")
    hasVowel, hasConstenant := false, false
    for _, r := range word {
        if !unicode.IsDigit(r) && !unicode.IsLetter(r) {
            return false
        }

        if slices.Contains(vowels, r) {
            hasVowel = true
        } else if unicode.IsLetter(r) {
            hasConstenant = true
        }
    }

    return hasVowel && hasConstenant
}