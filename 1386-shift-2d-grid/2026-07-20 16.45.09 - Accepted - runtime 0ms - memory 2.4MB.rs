impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let (m, n) = (grid.len(), grid[0].len());
        let s = m * n;

        grid.into_iter()
            .flatten()
            .cycle()
            .skip(s - k % s)
            .take(s)
            .enumerate()
            .fold(
                vec![vec![0; n]; m], 
                |mut res, (i, x)| {
                    res[i / n][i % n] = x;
                    res
                }
            )
    }
}