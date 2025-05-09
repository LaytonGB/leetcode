impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];
        res[0][0] = 1;
        let (mut top, mut left, mut bottom, mut right) = (0, 0, n, n);
        let mut m = 2;
        let (mut r, mut c) = (0, 0);
        while top < bottom || left < right {
            for w in [0, 1, 0, !1 + 1, 0].windows(2) {
                let (v, h) = (w[0], w[1]);
                while r + v >= top && r + v < bottom && c + h >= left && c + h < right {
                    r += v;
                    c += h;
                    res[r][c] = m;
                    m += 1;
                }
                if v == 0 {
                    if h == 1 {
                        top += 1;
                    } else {
                        bottom -= 1;
                    }
                } else if v == 1 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        res
    }
}