impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (r, c) = (matrix.len(), matrix[0].len());
        
        let mut row_zero_set = vec![false; r];
        let mut col_zero_set = vec![false; c];

        for i in 0..r {
            for j in 0..c {
                if matrix[i][j] == 0 {
                    row_zero_set[i] = true;
                    col_zero_set[j] = true;
                }
            }
        }

        Self::set(matrix, (r, c), row_zero_set, col_zero_set);
    }

    fn set(matrix: &mut Vec<Vec<i32>>, (r, c): (usize, usize), rows: Vec<bool>, cols: Vec<bool>) {
        for (i, _) in rows.into_iter().enumerate().filter(|(_, x)| *x) {
            for j in 0..c {
                matrix[i][j] = 0;
            }
        }

        for (j, _) in cols.into_iter().enumerate().filter(|(_, x)| *x) {
            for i in 0..r {
                matrix[i][j] = 0;
            }
        }
    }
}