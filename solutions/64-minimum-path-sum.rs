impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut g = vec![vec![grid[0][0]; m]; n];
        for i in 1..n {
            g[i][0] = g[i - 1][0] + grid[i][0];
        }
        for j in 1..m {
            g[0][j] = g[0][j - 1] + grid[0][j];
        }
        for i in 1..n {
            for j in 1..m {
                g[i][j] = g[i][j - 1].min(g[i - 1][j]) + grid[i][j];
            }
        }
        g[n - 1][m - 1]
    }
}