impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0_i32;
        let mut h = nums.len() as i32 - 1;
        let mut m: i32;
        while l < h {
            m = (l + h) / 2;
            if nums[m as usize] < target {
                l = m + 1;
            } else {
                h = m;
            }
        }
        if nums[l as usize] == target {
            return l;
        }
        -1
    }
}