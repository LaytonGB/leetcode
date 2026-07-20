impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut max = 0;
        for d in gain {
            cur += d;
            max = max.max(cur);
        }
        max
    }
}