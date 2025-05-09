from collections import Counter

class Solution:
    def findSubstring(self, s: str, words: List[str]) -> List[int]:
        words_len = len(words[0])
        if len(s) < len(words) * words_len:
            return []
        count = Counter(words)
        out = []
        for i in range(len(s) - words_len + 1):
            if count.get(s[i:i+words_len], 0) > 0:
                j = i + words_len
                this_count = count.copy()
                n = this_count.get(s[i:j], 0)
                if n == 1:
                    this_count.pop(s[i:j])
                else:
                    this_count[s[i:j]] -= 1
                while len(this_count) > 0:
                    n = this_count.get(s[j:j+words_len], 0)
                    if n == 1:
                        this_count.pop(s[j:j+words_len])
                    elif n > 0:
                        this_count[s[j:j+words_len]] -= 1
                    else:
                        break
                    j += words_len
                if len(this_count) == 0:
                    out.append(i)
        return out