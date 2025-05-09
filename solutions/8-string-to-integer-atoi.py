class Solution:
    def myAtoi(self, s: str) -> int:
        lim_low = -2147483648
        lim_high = 2147483647
        neg = False
        n = []
        
        i = 0
        
        try:
            while s[i] == " ":
                i += 1
            if s[i] == "-":
                neg = True
            if s[i] in ["-", "+"]:
                i += 1

            while True:
                int(s[i])
                n.append(s[i])
                i += 1
        except (IndexError, ValueError) as e:
            pass
        
        if len(n) < 1:
            return 0
        
        out = int("".join(n))
        if neg:
            out *= -1
        if out < lim_low:
            out = lim_low
        elif out > lim_high:
            out = lim_high
        return out
        