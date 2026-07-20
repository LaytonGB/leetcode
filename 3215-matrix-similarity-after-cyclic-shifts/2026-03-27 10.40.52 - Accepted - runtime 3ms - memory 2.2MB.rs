use itertools::Either;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let k = k as usize;
        let (m, n) = (mat.len(), mat[0].len());

        for i in 0..m {
            let mut it = if i % 2 == 0 {
                Either::Left((0..n).map(|j| (j + k) % n))
            } else {
                Either::Right((0..n).map(|j| (j + n * 50 - k) % n))
            };

            for (a, b) in (0..n).zip(it) {
                if mat[i][a] != mat[i][b] {
                    return false;
                }
            }
        }

        true
    }
}