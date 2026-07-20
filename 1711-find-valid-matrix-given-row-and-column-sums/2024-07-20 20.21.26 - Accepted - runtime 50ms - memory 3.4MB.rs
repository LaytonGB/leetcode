impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let (r_len, c_len) = (row_sum.len(), col_sum.len());
        let (mut r, mut c) = (0, 0);
        let mut res = vec![vec![0_i32; c_len]; r_len];
        while r < r_len && c < c_len {
            res[r][c] = row_sum[r].min(col_sum[c]);
            row_sum[r] -= res[r][c];
            col_sum[c] -= res[r][c];

            if row_sum[r] == 0 {
                r += 1;
            }
            if col_sum[c] == 0 {
                c += 1;
            }
        }
        res
    }
}