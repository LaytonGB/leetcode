impl Solution {
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (h, w) = (grid.len(), grid[0].len());
        let mut y_sum_grid = vec![vec![0; w]; h];
        // println!("y_sum_grid init: {:?}", y_sum_grid);
        
        let mut res = 0;
        for y in 0..h {
            for x in 0..w {
                if x > 0 {
                    grid[y][x] = grid[y][x] + grid[y][x-1];
                }
                if y > 0 {
                    y_sum_grid[y][x] = grid[y][x] + y_sum_grid[y-1][x];
                } else {
                    y_sum_grid[y][x] = grid[y][x];
                }

                if y_sum_grid[y][x] > k {
                    // println!("reject {}:{} | res: {} | grid: {}", y, x, res, y_sum_grid[y][x]);
                    break;
                }

                // println!("kept {}:{} | res: {} | grid: {}", y, x, res, y_sum_grid[y][x]);
                res += 1;
            }
        }

        // println!("finished grid: {:?}", grid);
        // println!("finished y_sum_grid: {:?}", y_sum_grid);

        res
    }
}