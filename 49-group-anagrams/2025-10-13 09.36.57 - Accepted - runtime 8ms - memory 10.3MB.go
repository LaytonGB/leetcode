func groupAnagrams(strs []string) [][]string {
    set := make(map[[26]int]int)
    var res [][]string
    for _, s := range strs {
        var count [26]int
        for i := 0; i < len(s); i++ {
            count[s[i]-'a']++
        }
        
        if idx, ok := set[count]; ok {
            res[idx] = append(res[idx], s)
        } else {
            set[count] = len(res)
            res = append(res, []string{s})
        }
    }
    return res
}