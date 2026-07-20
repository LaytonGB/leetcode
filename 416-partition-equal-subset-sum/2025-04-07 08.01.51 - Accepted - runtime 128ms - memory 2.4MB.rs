use std::collections::HashSet;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum & 1 == 1 {
            return false;
        }

        let tgt = sum / 2;
        let mut memo = HashSet::from([0]);

        for x in nums.iter() {
            let sums: Vec<i32> = memo.iter().copied().collect();
            sums.into_iter().for_each(|s| { memo.insert(s + x); });
        }

        memo.contains(&tgt)
    }
}