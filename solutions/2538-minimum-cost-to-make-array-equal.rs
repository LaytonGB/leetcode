impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let (mut l, mut h) = nums.iter().fold((i32::MAX, i32::MIN), |(l, h), &x| {
            (l.min(x), h.max(x))
        });
        let mut res = Self::calc_cost(&nums, &cost, l);
        while l < h {
            let m = (l + h) / 2;
            let (y1, y2) = (Self::calc_cost(&nums, &cost, m), Self::calc_cost(&nums, &cost, m + 1));
            res = y1.min(y2);
            if y1 < y2 {
                h = m;
            } else {
                l = m + 1;
            }
        }
        res
    }

    fn calc_cost(nums: &Vec<i32>, cost: &Vec<i32>, target: i32) -> i64 {
        let n = nums.len();
        (0..n).fold(0_i64, |sum, i| sum + ((nums[i] - target).abs() as i64 * cost[i] as i64))
    }
}