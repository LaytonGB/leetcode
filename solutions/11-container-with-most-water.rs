use std::cmp::Ordering as O;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut res = 0;

        while l < r {
            res = res.max(height[l].min(height[r]) * l.abs_diff(r) as i32);

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        res
    }
}