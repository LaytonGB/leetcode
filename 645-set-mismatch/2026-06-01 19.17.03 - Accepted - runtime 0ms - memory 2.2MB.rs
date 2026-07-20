impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut found = vec![false; nums.len()];
        let mut double = 0;
        for x in nums.into_iter().map(|x| x as usize - 1) {
            if found[x] {
                double = x as i32 + 1;
            } else {
                found[x] = true;
            }
        }
        let missing = found.into_iter()
            .enumerate()
            .find(|(i, x)| !x)
            .unwrap()
            .0 as i32 + 1;
        vec![double, missing]
    }
}