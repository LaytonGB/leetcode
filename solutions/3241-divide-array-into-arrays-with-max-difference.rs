impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        if nums.chunks(3).map(|c| c[2] - c[0]).max().unwrap() > k {
            return vec![];
        }
        nums.chunks(3).map(|c| c.to_vec()).collect()
    }
}