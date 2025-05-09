class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        rad = len(matrix) // 2
        if rad < 1:
            return
        for i in range(rad):
            for start_row in range(len(matrix)-2*i-1):
                positions = [
                    (i, start_row+i),
                    (start_row+i, len(matrix)-i-1),
                    (len(matrix)-i-1, len(matrix)-start_row-1-i),
                    (len(matrix)-start_row-1-i, i),
                    (i, start_row+i),
                ]
                n1 = None
                n2 = None
                for (r,c) in positions:
                    n2 = matrix[r][c]
                    matrix[r][c] = n1
                    n1 = n2