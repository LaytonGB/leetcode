impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|x| x.ilog10() + 1)
            .filter(|x| x % 2 == 0)
            .count() as i32
    }
}