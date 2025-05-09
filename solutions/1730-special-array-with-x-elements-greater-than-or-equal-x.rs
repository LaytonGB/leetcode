impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut counts = [0; 1001];
        for x in nums.into_iter() {
            counts[x as usize] += 1;
        }

        let mut sum = 0;
        for (i, c) in counts.into_iter().enumerate().rev() {
            sum += c;
            if i == sum {
                return i as i32;
            }
        }

        -1
    }
}