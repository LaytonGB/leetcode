public class Solution {
    public int[][] RestoreMatrix(int[] rowSum, int[] colSum) {
        int rows = rowSum.Length, cols = colSum.Length;
        int r = 0, c = 0;
        int[][] res = new int[rows][];
        for (int i = 0; i < rows; i++) {
            res[i] = new int[cols];
        }

        while (r < rows && c < cols) {
            res[r][c] = Math.Min(rowSum[r], colSum[c]);
            rowSum[r] -= res[r][c];
            colSum[c] -= res[r][c];

            if (rowSum[r] == 0) {
                r += 1;
            }
            if (colSum[c] == 0) {
                c += 1;
            }
        }
        return res;
    }
}