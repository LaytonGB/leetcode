impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let row_sum_iter = grid.iter()
            .map(|row| row.iter().map(|x| *x as i64).sum::<i64>());
        println!("row_sum_iter {:?}", row_sum_iter.clone().collect::<Vec<i64>>());
        
        let total = row_sum_iter.clone().sum::<i64>();
        
        if total % 2 == 1 {
            return false;
        }

        let half = total / 2;

        let mut prefix = 0;
        for row_sum in row_sum_iter {
            prefix += row_sum;
            if prefix == half {
                return true;
            }
        }

        let col_sum_iter = (0..grid[0].len())
            .map(|j| (0..grid.len()).map(|i| grid[i][j] as i64).sum::<i64>());
        println!("col_sum_iter {:?}", col_sum_iter.clone().collect::<Vec<i64>>());
        
        prefix = 0;
        for col_sum in col_sum_iter {
            prefix += col_sum;
            if prefix == half {
                return true;
            }
        }

        false
    }
}