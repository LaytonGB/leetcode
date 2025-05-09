const DIRS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut res = 0_i32;
        for i in 0..m {
            for j in 0..n {
                let total = Self::collect(&mut grid, m, n, i, j);
                res = res.max(total);
            }
        }

        res
    }

    fn collect(grid: &mut Vec<Vec<i32>>, m: usize, n: usize, i: usize, j: usize) -> i32 {
        if i >= m || j >= n || grid[i][j] == 0 {
            return 0;
        }
        
        let mut res = grid[i][j];
        grid[i][j] = 0;

        let mut max = 0;
        for (r, c) in DIRS {
            let (i, j) = (i + r, j + c);
            max = max.max(Self::collect(grid, m, n, i, j));
        }

        grid[i][j] = res;
        res + max
    }
}