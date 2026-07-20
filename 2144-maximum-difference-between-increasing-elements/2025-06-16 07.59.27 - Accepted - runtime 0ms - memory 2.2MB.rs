impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut low = nums[0];
        let mut res = 0;
        for x in nums.into_iter().skip(1) {
            if x < low {
                low = x;
            } else {
                res = res.max(x - low);
            }
        }

        if res <= 0 {
            -1
        } else {
            res
        }
    }
}