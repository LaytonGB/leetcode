impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        nums.sort_unstable();
        while nums.binary_search(&original).is_ok() {
            original *= 2;
        }
        original
    }
}