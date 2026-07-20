impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut res = vec![0; nums.len()];
        for i in 0..n {
            let idx = (i + nums[i as usize] + n * 100) % n;
            res[i as usize] = nums[idx as usize];
        }
        res
    }
}