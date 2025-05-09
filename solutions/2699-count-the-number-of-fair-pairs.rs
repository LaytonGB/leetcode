impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let count_less_than = |limit: i32| {
            let mut res = 0;
            let (mut i, mut j) = (0, nums.len() - 1);
            while i < j {
                if nums[i] + nums[j] > limit {
                    j -= 1;
                } else {
                    res += j - i;
                    i += 1;
                }
            }

            res as i64
        };

        count_less_than(upper) - count_less_than(lower - 1)
    }
}