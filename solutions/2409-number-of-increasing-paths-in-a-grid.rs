const MOD: i64 = 10_i64.pow(9) + 7;
impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut res = 0_i64;
        let mut vis = vec![vec![false; n]; m];
        let mut cache = vec![vec![None; n]; m];
        for i in 0..m {
            for j in 0..n {
                res = (res + Self::dfs(&grid, &mut vis, &mut cache, (i,j))) % MOD;
            }
        }
        res as i32
    }

    fn dfs(grid: &Vec<Vec<i32>>, vis: &mut Vec<Vec<bool>>, cache: &mut Vec<Vec<Option<i64>>>, (i,j): (usize, usize)) -> i64 {
        if let Some(res) = cache[i][j] {
            return res;
        }
        vis[i][j] = true;
        let mut res = 1;
        for w in [0, 1, 0, !1 + 1, 0].windows(2) {
            let (r, c) = (i + w[0], j + w[1]);
            if r < grid.len() && c < grid[0].len() && !vis[r][c] && grid[r][c] > grid[i][j] {
                res = (res + Self::dfs(grid, vis, cache, (r,c))) % MOD;
            }
        }
        vis[i][j] = false;
        cache[i][j] = Some(res);
        res
    }
}