impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        // toggle rows and cols to best possible values
        for i in 0..m {
            if grid[i][0] != 1 {
                grid[i].iter_mut().for_each(|x| *x = (*x + 1) % 2);
            }
        }
        for j in 1..n {
            if (0..m).fold(0_usize, |ones, i| ones + (grid[i][j] == 1) as usize) < (m + 1) / 2 {
                (0..m).for_each(|i| grid[i][j] = (grid[i][j] + 1) % 2);
            }
        }

        // tally row and col numbers
        grid.iter().map(|r| r.iter().fold(0_i32, |a, b| a * 2 + b)).sum()
    }
}