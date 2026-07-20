impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        
        let row_prefix = grid.iter()
            .map(|row| row.iter().map(|x| *x as i64).sum::<i64>())
            .scan(0, |mut prev, curr| { *prev += curr; Some(*prev)})
            .collect::<Vec<i64>>();
        // println!("row_prefix {:?}", row_prefix);

        let row_prefix_rev = grid.iter().rev()
            .map(|row| row.iter().map(|x| *x as i64).sum::<i64>())
            .scan(0, |mut prev, curr| { *prev += curr; Some(*prev)})
            .collect::<Vec<i64>>();
        // println!("row_prefix_rev {:?}", row_prefix_rev);

        for i in 0..m-1 {
            // println!("{} - {} = {}", row_prefix[i], row_prefix_rev[m-i-2], row_prefix[i] - row_prefix_rev[m-i-2]);
            if row_prefix[i] - row_prefix_rev[m-i-2] == 0 {
                return true;
            }
        }

        let col_prefix = (0..n)
            .map(|j| (0..m).map(|i| grid[i][j]).map(|x| x as i64).sum::<i64>())
            .scan(0, |mut prev, curr| { *prev += curr; Some(*prev)})
            .collect::<Vec<i64>>();
        // println!("col_prefix {:?}", col_prefix);

        let col_prefix_rev = (0..n).rev()
            .map(|j| (0..m).map(|i| grid[i][j]).map(|x| x as i64).sum::<i64>())
            .scan(0, |mut prev, curr| { *prev += curr; Some(*prev)})
            .collect::<Vec<i64>>();
        // println!("col_prefix_rev {:?}", col_prefix_rev);

        for i in 0..n-1 {
            // println!("{} - {} = {}", col_prefix[i], col_prefix_rev[n-i-2], col_prefix[i] - col_prefix_rev[n-i-2]);
            if col_prefix[i] - col_prefix_rev[n-i-2] == 0 {
                return true;
            }
        }
        
        false
    }
}