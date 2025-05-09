// https://leetcode.com/problems/magnetic-force-between-two-balls/solutions/794070/python-c-binary-search-solution-with-explanation-and-similar-questions
impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        
        // binary search for ideal d
        // count too high == d too low, and vice versa
        let (mut l, mut h) = (0, *position.last().unwrap() - position[0]);
        while l < h {
            let d = h - (h - l) / 2;
            let c = Self::count(&position, d);
            if c >= m {
                l = d;
            } else {
                h = d - 1;
            }
        }

        l
    }

    // count all ball positions with at least gap d between balls
    fn count(position: &[i32], d: i32) -> i32 {
        let mut res = 1;
        let mut curr = position[0];
        for i in 1..position.len() {
            if position[i] - curr >= d {
                res += 1;
                curr = position[i];
            }
        }
        res
    }
}