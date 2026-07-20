impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());

        let mut res = 0;
        for i in 0..m {
            let Some(j) = Self::get_idx_of_row_one_if_singular(&mat, i) else { continue; };
            if Self::get_idx_of_col_one_if_singular(&mat, j).is_none() { continue; };
            res += 1;
        }

        res
    }

    fn get_idx_of_row_one_if_singular(mat: &Vec<Vec<i32>>, row_idx: usize) -> Option<usize> {
        let mut one_idx = None;
        for j in 0..mat[0].len() {
            if mat[row_idx][j] == 1 {
                if one_idx.is_none() {
                    one_idx = Some(j);
                } else {
                    return None;
                }
            }
        }
        one_idx
    }

    fn get_idx_of_col_one_if_singular(mat: &Vec<Vec<i32>>, col_idx: usize) -> Option<usize> {
        let mut one_idx = None;
        for i in 0..mat.len() {
            if mat[i][col_idx] == 1 {
                if one_idx.is_some() {
                    return None;
                } else {
                    one_idx = Some(i);
                }
            }
        }
        one_idx
    }
}