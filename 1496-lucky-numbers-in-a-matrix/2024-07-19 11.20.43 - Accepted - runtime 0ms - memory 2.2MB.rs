impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row_minimums = matrix.iter().map(|r| *r.iter().min().unwrap()).collect::<Vec<_>>();
        let col_maximums = (0..matrix[0].len())
            .map(|i| (0..matrix.len()).map(|j| matrix[j][i]).max().unwrap())
            .collect::<Vec<_>>();
        
        row_minimums.into_iter()
            .filter(|r| col_maximums.contains(r))
            .collect()
    }
}