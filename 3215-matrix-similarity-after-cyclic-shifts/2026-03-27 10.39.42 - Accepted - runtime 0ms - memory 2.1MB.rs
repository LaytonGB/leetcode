impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let k = k as usize;
        let (m, n) = (mat.len(), mat[0].len());

        for i in 0..m {
            let offset = if i % 2 == 0 {
                k
            } else {
                n * 50 - k
            };

            for (a, b) in (0..n).zip((0..n).map(|j| (j + offset) % n)) {
                if mat[i][a] != mat[i][b] {
                    return false;
                }
            }
        }

        true
    }
}