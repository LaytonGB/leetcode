impl Solution {
    pub fn check(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        nums.extend(nums.clone());
'outer: for i in 0..n {
            let mut prev = 0;
            for j in 0..n {
                let k = i + j;
                if prev > nums[k] {
                    continue 'outer;
                }
                prev = nums[k];
            }
            return true;
        }
        false
    }
}