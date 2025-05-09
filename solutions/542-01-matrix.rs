use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut res = vec![vec![i32::MAX; n]; m];
        let mut q = VecDeque::new();
        for (i, row) in mat.into_iter().enumerate() {
            for (j, x) in row.into_iter().enumerate() {
                if x == 0 {
                    res[i][j] = x;
                    q.push_back((i,j));
                }
            }
        }
        while let Some((i,j)) = q.pop_front() {
            for w in [0,1,0,!0,0].windows(2) {
                let (r,c) = (i.wrapping_add(w[0]), j.wrapping_add(w[1]));
                if r < m && c < n && res[r][c] > res[i][j] { // update values that have found shorter paths only
                    res[r][c] = res[i][j] + 1;
                    q.push_back((r,c)); // see if adj spaces can benefit from new position
                }
            }
        }
        res
    }
}