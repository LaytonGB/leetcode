// Adapted from https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/solutions/7631171/super-easy-full-clear-explanation-beginn-lpcn

func minFlips(s string) int {
    n := len(s)
    s = s + s

    a, b := 0, 0
    res := int(^uint(0) >> 1) // start with max int
    for l, r := 0, 0; r < n * 2; r++ {
        if uint8(r % 2) + '0' == s[r] {
            b++
        } else {
            a++
        }

        if r - l + 1 > n {
            if uint8(l % 2) + '0' == s[l] {
                b--
            } else {
                a--
            }
            l++
        }

        if r - l + 1 == n {
            res = min(res, a, b)
        }
    }

    return res
}