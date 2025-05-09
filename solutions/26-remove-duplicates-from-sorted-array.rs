impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut s, mut f) = (1, 1);
        while f < nums.len() {
            if nums[s - 1] != nums[f] {
                nums[s] = nums[f];
                s += 1;
            }
            f += 1;
        }
        s as i32
    }
}