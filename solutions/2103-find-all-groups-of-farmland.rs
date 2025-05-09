impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (land.len(), land[0].len());
        let mut res = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if land[i][j] == 1 {
                    let (mut r, mut c) = (i, j);
                    while r < m && land[r][j] == 1 {
                        r += 1;
                    }
                    while c < n && land[i][c] == 1 {
                        c += 1;
                    }
                    Self::mark(&mut land, i, j, r, c);
                    res.push(vec![i as i32, j as i32, r as i32 - 1, c as i32 - 1]);
                }
            }
        }
        res
    }

    fn mark(land: &mut Vec<Vec<i32>>, i: usize, j: usize, r: usize, c: usize) {
        for i in i..r {
            for j in j..c {
                land[i][j] = 0;
            }
        }
    }
}