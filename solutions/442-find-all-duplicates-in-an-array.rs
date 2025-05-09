impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = [0; 100_000];
        let mut res = Vec::new();
        for n in nums.iter() {
            if seen[*n as usize - 1] == 1 {
                res.push(*n);
            } else {
                seen[*n as usize - 1] += 1;
            }
        }
        res
    }
}