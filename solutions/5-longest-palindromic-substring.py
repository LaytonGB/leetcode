class Solution:
    # Thank you geeksforgeeks \U0001f64f
    # Manchers Algorithm is wild, the man is a champ
    # https://www.geeksforgeeks.org/manachers-algorithm-linear-time-longest-palindromic-substring-part-1/
    def longestPalindrome(self, s: str) -> str:
        if len(s) < 2:
            return s
        
        n = 2 * len(s) + 1
        lps = [0] * n
        
        c = 1 # center position
        d = 2 # center right
        
        max_lps = 0
        max_lps_idx = 0
        
        # for each currentRight
        for i in range(0, n):
            diff = d - i
            # if currentRight within centerRight
            if diff > 0:
                lps[i] = min(lps[2 * c - i], diff) # use smaller of centerLeft value or d-i
                
            # attempt expansion from c+i (current right)
            # odd positions are characters - compare them
            # even positions are fillers, just increment
            try:
                while (i + lps[i] < n and i - lps[i] > 0) and \
                    ((i + lps[i] + 1) % 2 == 0 or \
                    s[(i + lps[i] + 1) // 2] == s[(i - lps[i] - 1) // 2]):
                    lps[i] += 1
            except IndexError as e:
                pass
            
            if lps[i] > max_lps:
                max_lps = lps[i]
                max_lps_idx = i
            
            # expand and adjust centerRight and center based on currentRight
            new_d = i + lps[i]
            if new_d > d:
                c = i
                d = new_d
                
        lps_start = (max_lps_idx - max_lps) // 2
        print(lps, max_lps_idx, max_lps, "->", lps_start, lps_start + max_lps)
        return s[lps_start : lps_start + max_lps]