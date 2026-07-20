use std::collections::VecDeque;
impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        let n = nums.len() / 2;
        nums.sort_unstable();
        let mut vd = VecDeque::from(nums);
        let mut res = i32::MAX;
        for i in 0..n {
            let a = vd.pop_front().unwrap();
            let b = vd.pop_back().unwrap();
            res = res.min(a + b);
        }
        res as f64 / 2.0
    }
}