impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut z = 0;
        nums.retain(|n| if *n == 0 {z += 1; false} else {true});
        &nums.append(&mut vec![0; z]);
    }
}