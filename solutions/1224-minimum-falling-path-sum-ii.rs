impl Solution {
    pub fn min_falling_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        for i in 1..n {
            let r = grid[i - 1].iter().fold([i32::MAX; 2], |mut r, &x| {
                if x < r[0] {
                    r.swap(0, 1);
                    r[0] = x;
                } else if x < r[1] {
                    r[1] = x;
                }
                r
            });

            for j in 0..n {
                grid[i][j] += if grid[i - 1][j] == r[0] {
                    r[1]
                } else {
                    r[0]
                };
            }
        }

        *grid[n - 1].iter().min().unwrap()
    }
}