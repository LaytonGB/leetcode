impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let counts = nums.iter().fold([0_u8; 201], |mut m, n| {
            m[(n + 100) as usize] += 1;
            m
        });
        
        nums.sort_unstable_by_key(|n| (counts[(n + 100) as usize], -n));

        nums
    }
}