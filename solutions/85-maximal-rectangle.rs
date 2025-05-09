impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());

        let mut heights = vec![0; n + 1];
        let mut res = 0;

        for i in 0..m {
            for j in 0..n {
                heights[j] = if matrix[i][j] == '1' {
                    heights[j] + 1
                } else {
                    0
                };
            }

            let mut v = Vec::with_capacity(n + 1);
            for i in 0..=n {
                while !v.is_empty() && heights[i] < heights[*v.last().unwrap()] {
                    let h = heights[v.pop().unwrap()];
                    let w = if v.is_empty() {
                        i
                    } else {
                        i - *v.last().unwrap() - 1
                    };
                    res = res.max(h * w);
                }
                v.push(i);
            }
        }

        res as i32
    }
}