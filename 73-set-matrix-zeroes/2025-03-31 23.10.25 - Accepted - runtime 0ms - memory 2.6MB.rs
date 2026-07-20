use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut update_rows = HashSet::new();
        let mut update_cols = HashSet::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    update_rows.insert(i);
                    update_cols.insert(j);
                }
            }
        }

        for row in update_rows.into_iter() {
            for j in 0..matrix[row].len() {
                matrix[row][j] = 0;
            }
        }
        for col in update_cols.into_iter() {
            for i in 0..matrix.len() {
                matrix[i][col] = 0;
            }
        }
    }
}