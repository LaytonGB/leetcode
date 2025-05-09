public class Solution {
    public IList<int> LuckyNumbers (int[][] matrix) {
        var min_row = matrix.Select(row => row.Min()).ToList();
        var max_col = Enumerable.Range(0, matrix[0].Count())
            .Select(j => Enumerable.Range(0, matrix.Count())
                .Select(i => matrix[i][j]))
            .Select(col => col.Max())
            .ToList();
        return min_row.Where(x => max_col.Contains(x)).ToList();
    }
}