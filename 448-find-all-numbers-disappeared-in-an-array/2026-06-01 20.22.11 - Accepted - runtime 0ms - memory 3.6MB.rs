impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = vec![false; nums.len()];
        for x in nums {
            seen[x as usize - 1] = true;
        }
        seen.into_iter()
            .enumerate()
            .filter(|(_, x)| !x)
            .map(|(i, _)| i as i32 + 1)
            .collect()
    }
}