impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row_minimums = matrix.iter().map(|r| *r.iter().min().unwrap()).collect::<Vec<_>>();
        let is_col_maximum = (0..matrix[0].len())
            .fold([false; 100001], |mut res, i| {
                res[(0..matrix.len()).map(|j| matrix[j][i]).max().unwrap() as usize] = true;
                res
            });
        
        row_minimums.into_iter()
            .filter(|&r| is_col_maximum[r as usize])
            .collect()
    }
}