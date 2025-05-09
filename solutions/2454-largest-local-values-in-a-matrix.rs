const G: [(usize, usize); 9] = [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)];

impl Solution {
    pub fn largest_local(g: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = g.len();

        let mut res = vec![vec![0; n - 2]; n - 2];
        for i in 0..n - 2 {
            for j in 0..n - 2 {
                res[i][j] = G.iter().map(|(r,c)| g[i+r][j+c]).max().unwrap();
            }
        }

        res
    }
}