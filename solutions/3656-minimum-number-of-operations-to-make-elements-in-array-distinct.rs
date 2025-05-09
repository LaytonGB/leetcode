impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut found = [false; 100];
        let mut last_removal_index = -1;
        for (i, x) in nums.iter().enumerate().rev() {
            if found[*x as usize - 1] {
                last_removal_index = i as i32;
                break;
            }

            found[*x as usize - 1] = true;
        }

        if last_removal_index == -1 {
            0
        } else {
            last_removal_index / 3 + 1
        }
    }
}