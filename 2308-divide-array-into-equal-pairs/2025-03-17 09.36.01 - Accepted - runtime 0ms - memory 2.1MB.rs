impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        nums.into_iter()
            .fold([true; 500], |mut counts, n| {
                counts[(n - 1) as usize] = !counts[(n - 1) as usize];
                counts
            })
            .into_iter()
            .all(|c| c)
    }
}