impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = (0, 0);
        while dp.1 < nums.len() - 1 {
            let next = (dp.0 ..= dp.1).into_iter().map(|i| i + nums[i] as usize).collect::<Vec<usize>>();
            dp = (dp.1, next.into_iter().max().unwrap());
            res += 1;
        }
        res
    }
}