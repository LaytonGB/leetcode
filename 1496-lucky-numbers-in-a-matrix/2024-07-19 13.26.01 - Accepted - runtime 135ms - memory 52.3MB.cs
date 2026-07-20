public class Solution {
    public IList<int> LuckyNumbers (int[][] matrix) {
        var min_row = new List<int>();
        var max_col = new List<int>();
        for (var i = 0; i < matrix.Count(); i++) {
            min_row.Add(matrix[i].Min());
        }

        for (var j = 0; j < matrix[0].Count(); j++) {
            var temp_col = new List<int>();
            for (var i = 0; i < matrix.Count(); i++) {
                temp_col.Add(matrix[i][j]);
            }
            max_col.Add(temp_col.Max());
        }

        var res = new List<int>();
        foreach (var x in min_row) {
            if (max_col.Contains(x)) {
                res.Add(x);
            }
        }

        return res;
    }
}