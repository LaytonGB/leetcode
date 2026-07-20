impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut count = [true; 500];
        let mut pair_count = 0;

        for n in nums.into_iter() {
            count[(n - 1) as usize] = !count[(n - 1) as usize];
            if count[(n - 1) as usize] {
                pair_count += 1;
            }
        }

        pair_count == n / 2
    }
}