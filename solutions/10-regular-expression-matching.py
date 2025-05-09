import re

class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        m = {}
        def match_from(i, j):
            if (i, j) in m:
                return m[i, j]
            if j == len(p):
                out = i == len(s)
            else:
                first_match = i < len(s) and p[j] in {s[i], "."}
                if j+1 < len(p) and p[j+1] == "*":
                    out = match_from(i, j+2) or first_match and match_from(i+1, j)
                else:
                    out = first_match and match_from(i+1, j+1)
                    
            m[i, j] = out
            return out
        return match_from(0, 0)