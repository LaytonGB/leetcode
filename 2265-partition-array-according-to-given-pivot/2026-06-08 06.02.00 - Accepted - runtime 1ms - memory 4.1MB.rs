impl Solution {
    pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        nums.sort_by_key(|x| x.cmp(&pivot));
        nums
    }
}