impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by_key(|n| {
            if *n == 0 {
                return mapping[0];
            }
            
            let mut m = 0_i32;
            for i in 0..=(*n).ilog10() {
                m += mapping[((*n / 10_i32.pow(i)) % 10) as usize] * 10_i32.pow(i);
            }
            m
        });
        nums
    }
}