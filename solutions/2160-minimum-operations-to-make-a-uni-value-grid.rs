impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut factors = vec![0; m * n];
        let offset = grid[0][0] % x;
        for (i, r) in grid.iter().enumerate() {
            for (j, v) in r.iter().enumerate() {
                if *v % x != offset {
                    return -1;
                }

                factors[i * n + j] = *v / x;
            }
        }

        factors.sort_unstable();
        
        let median = factors[m * n / 2];

        factors.iter().fold(0, |res, f| {
            res + (f - median).abs()
        })
    }
}