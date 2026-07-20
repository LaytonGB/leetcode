use std::cmp::Ordering as O;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let res = Self::h(&grid, grid.len(), grid[0].len(), 0, 0, 1);

        if res < 0 {
            return -1;
        }

        (res % MOD) as i32
    }

    fn h(grid: &Vec<Vec<i32>>, m: usize, n: usize, r: usize, c: usize, score: i64) -> i64 {
        if grid[r][c] == 0 {
            return 0;
        }
        
        match (r.cmp(&(m - 1)), c.cmp(&(n - 1))) {
            (O::Less, O::Less) => {
                Self::h(grid, m, n, r + 1, c, score * grid[r][c] as i64)
                    .max(Self::h(grid, m, n, r, c + 1, score * grid[r][c] as i64))
            }
            (O::Less, _) => {
                Self::h(grid, m, n, r + 1, c, score * grid[r][c] as i64)
            }
            (_, O::Less) => {
                Self::h(grid, m, n, r, c + 1, score * grid[r][c] as i64)
            }
            _ => {
                score * grid[r][c] as i64
            }
        }
    }
}