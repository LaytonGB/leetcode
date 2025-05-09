impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();

        let k_rad = k * 2 + 1;
        let mut res = vec![-1; n];
        if k_rad > n {
            return res;
        }

        let mut sum: usize = nums[0..k_rad-1].iter().map(|&x| x as usize).sum();
        for i in 0..=n-k_rad {
            sum += nums[i+k_rad-1] as usize;
            res[i+k] = (sum / k_rad as usize) as i32;
            sum -= nums[i] as usize;
        }
        res
    }
}