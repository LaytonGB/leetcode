const DIRS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    res += 1;
                    Self::fill(&mut grid, i, j);
                }
            }
        }
        res
    }

    fn fill(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] != '1' {
            return;
        }

        grid[i][j] = '0';
        for (di, dj) in DIRS {
            Self::fill(grid, i + di, j + dj);
        }
    }
}
