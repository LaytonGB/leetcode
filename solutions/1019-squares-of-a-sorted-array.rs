impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut l, mut r) = (0, n - 1);
        let mut res = vec![0; n];

        for i in (0..n).rev() {
            if nums[l].abs() >= nums[r].abs() {
                res[i] = nums[l].pow(2);
                l += 1;
            } else {
                res[i] = nums[r].pow(2);
                r -= 1;
            }
        }
        res
    }
}