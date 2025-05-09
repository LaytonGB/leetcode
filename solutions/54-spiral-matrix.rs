impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut top, mut left, mut bottom, mut right) = (0, 0, matrix.len(), matrix[0].len());
        let target = bottom * right;
        let (mut r, mut c) = (0, 0);
        let mut res = Vec::with_capacity(target);
        res.push(matrix[0][0]);
        loop {
            for dir in [0, 1, 0, !1 + 1, 0].windows(2) {
                let (v, h) = (dir[0], dir[1]);
                if res.len() >= target {
                    return res;
                }
                while r + v >= top && r + v < bottom && c + h >= left && c + h < right {
                    r = r + v;
                    c = c + h;
                    res.push(matrix[r][c]);
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
    }
}