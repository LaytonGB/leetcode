func makeFancyString(s string) string {
    if len(s) < 3 {
        return s
    }
    
    runes := []rune(s)
    res := runes[0:2]
    a := runes[0]
    b := runes[1]
    for _, c := range runes[2:] {
        if a != b || b != c  {
            res = append(res, c)
        }

        a = b
        b = c
    }

    return string(res)
}