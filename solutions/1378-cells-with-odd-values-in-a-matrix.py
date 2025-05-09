class Solution:
    def oddCells(self, m: int, n: int, indices: List[List[int]]) -> int:
        rows = [False] * m
        cols = [False] * n
        
        for r,c in indices:
            rows[r] = not rows[r]
            cols[c] = not cols[c]
            
        r = sum(rows)
        c = sum(cols)
        print(rows, cols, r, c)
        return r * n + c * m - 2 * r * c