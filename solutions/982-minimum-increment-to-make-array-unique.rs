impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut res = 0;
        let mut x = 0;
        for n in nums {
            res += 0.max(x - n);
            x = x.max(n) + 1;
        }
        res
    }
}