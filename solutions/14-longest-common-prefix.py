class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        p = strs[0]
        for s in strs[1:]:
            if len(p) > len(s):
                p = p[:len(s)]
            for i in range(min(len(p), len(s))):
                if p[i] != s[i]:
                    if i == 0:
                        return ""
                    p = p[:i]
                    break
        return p