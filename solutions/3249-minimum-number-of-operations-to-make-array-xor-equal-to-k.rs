impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let xor = nums.iter().copied().reduce(|a, b| a ^ b).unwrap();
        (xor ^ k).count_ones() as i32
    }
}