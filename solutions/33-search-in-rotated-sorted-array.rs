impl Solution {
    pub fn search(mut nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut h) = (0, nums.len() - 1);
        let mut m;
        println!("nums:{:?} | target:{:?}", nums, target);
        while l <= h {
            m = l + (h - l) / 2;
            if nums[m] == target {
                return m as i32;
            }
            if nums[m] >= nums[l] { // if pointer in left half (even if thats whole list)
                if target >= nums[l] && target < nums[m] {
                    h = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                if target <= nums[h] && target > nums[m] {
                    l = m + 1;
                } else {
                    h = m - 1;
                }
            }
        }
        -1
    }
}