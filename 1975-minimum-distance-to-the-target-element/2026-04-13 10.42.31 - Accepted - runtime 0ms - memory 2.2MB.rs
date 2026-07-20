impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let n = nums.len();

        let start = start as usize;
        if nums[start] == target {
            return 0;
        }

        for i in 1..n {
            if (start + i < n && nums[start+i] == target)
            || (start >= i && nums[start-i] == target) {
                return i as i32;
            }
        }

        unreachable!()
    }
}