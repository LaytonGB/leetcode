class Solution:
    def findAndReplacePattern(self, words: List[str], pattern: str) -> List[str]:
        def toDict(w):
            d = {l:i for i,l in enumerate(w[::-1])}

            return d
        
        def toPatt(w):
            m = toDict(w)
            return list(map(lambda l: m[l], w))
        
        p = toPatt(pattern)
        out = [w for w in words if len(w) == len(p) and toPatt(w) == p]
        return out