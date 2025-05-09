impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max_height = 0;
        let mut height = 0;
        for &d in gain.iter() {
            height += d;
            max_height = max_height.max(height);
        }
        max_height
    }
}