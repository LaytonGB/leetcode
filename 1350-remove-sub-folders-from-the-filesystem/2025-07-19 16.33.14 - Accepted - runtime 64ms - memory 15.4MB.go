func removeSubfolders(folder []string) []string {
    folders := getFoldersSet(folder)
    rootsSet := getRootsSet(folders)
    res := getSliceFromMap(rootsSet)
    return res
}

func getFoldersSet(folder [] string) map[string]struct{} {
    folders := make(map[string]struct{})
    for _, s := range folder {
        folders[s] = struct{}{}
    }
    return folders
}

func getRootsSet(folders map[string]struct{}) map[string]struct{} {
    rootsSet := make(map[string]struct{}, 0)
    for s := range folders {
        parts := strings.Split(s[1:], "/")
        partStr := ""
        for _, p := range parts {
            partStr += fmt.Sprintf("/%s", p)
            if _, ok := folders[partStr]; ok {
                rootsSet[partStr] = struct{}{}
                break
            }
        }
    }
    return rootsSet
}

func getSliceFromMap(m map[string]struct{}) []string {
    slice := make([]string, len(m))
    i := 0
    for k := range m {
        slice[i] = k
        i += 1
    }
    return slice
}