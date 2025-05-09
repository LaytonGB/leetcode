impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // move each nums[i] into the position idx nums[i] - 1
        for i in 0..n {
            let mut x = nums[i];
            let mut j = x as usize - 1;
            // while has some correct position and is not the same as adjacent num
            // do cycle through numbers until position correct
            while x > 0 && x as usize <= n && x != i as i32 + 1 && nums[j] != x {
                nums.swap(i, j);
                x = nums[i];
                j = x as usize - 1;
            }
        }

        // return first idx that wasnt in the correct position
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        
        n as i32 + 1
    }
}