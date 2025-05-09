impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut removals = 0;

        let mut i = 0;
        while i < nums.len() - 1 {
            match (nums[i], nums[i + 1]) {
                (0, _) => {
                    nums = [&nums[..i], &nums[i+1..]].concat();
                    removals += 1;
                }
                (a, b) if a == b => {
                    nums = [&nums[..i], &[nums[i] * 2], &nums[i+2..]].concat();
                    removals += 1;
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }

        nums.into_iter()
            .chain(std::iter::repeat(0).take(removals))
            .collect()
    }
}