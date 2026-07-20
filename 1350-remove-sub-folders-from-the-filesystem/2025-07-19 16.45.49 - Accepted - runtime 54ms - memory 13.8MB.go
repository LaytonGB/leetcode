func removeSubfolders(folder []string) []string {
    sort.Slice(folder, func(i, j int) bool {
        return len(folder[i]) < len(folder[j])
    })

    roots := make(map[string]struct{})
    outer:
    for _, s := range folder {
        parts := strings.Split(s[1:], "/")
        partsStr := ""
        for _, p := range parts {
            partsStr += fmt.Sprintf("/%s", p)
            if _, ok := roots[partsStr]; ok {
                continue outer
            }
        }
        roots[partsStr] = struct{}{}
    }

    res := make([]string, len(roots))
    i := 0
    for k := range roots {
        res[i] = k
        i += 1
    }

    return res
}