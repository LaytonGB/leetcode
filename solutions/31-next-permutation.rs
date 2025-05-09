impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if let Some(k) = (0..nums.len() - 1).rev().find(|&i| nums[i] < nums[i+1]) {
            let l = (k + 1..nums.len()).rev().find(|&i| nums[k] < nums[i]).unwrap();
            nums.swap(k, l);
            nums[k + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}