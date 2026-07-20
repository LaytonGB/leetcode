// Folders are always sorted with shortest name first.
// All adjacent folders have the most-similar directory names.
// Hence, if adjacent folders have an overlap in name, the first one should be kept.
func removeSubfolders(folder []string) []string {
    slices.Sort(folder)

    res := []string{ folder[0] }
    for i := 1; i < len(folder); i++ {
        // Appended forward slash ensures no prefixed namings
        // E.g. /a/b != /ab/b
        if !strings.HasPrefix(folder[i], res[len(res) - 1] + "/") {
            res = append(res, folder[i])
        }
    }

    return res
}