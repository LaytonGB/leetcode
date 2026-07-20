impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;

            if nums[m] > nums[r] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        nums[l]
    }
}