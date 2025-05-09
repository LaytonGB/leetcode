impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut row_sums = vec![-1_i64; m];
        let mut col_sums = vec![-1_i64; n];
        let mut res = 0;
        
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    row_sums[i] += 1;
                    col_sums[j] += 1;
                }
            }
        }
        
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    res += row_sums[i] * col_sums[j];
                }
            }
        }
        
        res
    }
}