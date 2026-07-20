impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        
        let mut xy = vec![0; n];
        let mut xz = vec![0; n];
        let mut yz = vec![0; n];

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }

                xy[i] += 1;
                xz[i] = xz[i].max(grid[i][j]);
                yz[j] = yz[j].max(grid[i][j]);
            }
        }

        xy.iter().sum::<i32>()
            + xz.iter().sum::<i32>()
            + yz.iter().sum::<i32>()
    }
}