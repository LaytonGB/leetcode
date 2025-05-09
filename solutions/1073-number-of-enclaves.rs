impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        
        // disregard any edge-touching enclaves
        for i in 0..m {
            if grid[i][0] == 1 {
                Self::dfs(&mut grid, i, 0);
            }
            if grid[i][n - 1] == 1 {
                Self::dfs(&mut grid, i, n - 1);
            }
        }
        for j in 0..n {
            if grid[0][j] == 1 {
                Self::dfs(&mut grid, 0, j);
            }
            if grid[m - 1][j] == 1 {
                Self::dfs(&mut grid, m - 1, j);
            }
        }

        // count remaining 1s
        let mut res = 0;
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                res += grid[i][j];
            }
        }
        res
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        grid[i][j] = 0;
        
        for w in [0, 1, 0, !1 + 1, 0].windows(2) {
            let (r, c) = (i.wrapping_add(w[0]), j.wrapping_add(w[1]));
            if r < grid.len() && c < grid[0].len() && grid[r][c] == 1 {
                Self::dfs(grid, r, c);
            }
        }
    }
}