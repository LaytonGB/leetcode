impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        let mut sum_left: Vec<i32> = vec![0; n];
        let mut sum_right: Vec<i32> = vec![0; n];
        sum_left[0] = nums[0];
        sum_right[n-1] = nums[n-1];
        for i in 1..n {
            sum_left[i] = sum_left[i-1] + nums[i];
            sum_right[n-i-1] = nums[n-i-1] + sum_right[n-i];
        }
        // println!("{:?} | {:?}", sum_left, sum_right);
        if 0 == sum_right[1] {
            return 0;
        }
        for i in 1..n-1 {
            if sum_left[i-1] == sum_right[i+1] {
                return i as i32;
            }
        }
        if sum_left[n-2] == 0 {
            return n as i32 - 1;
        }
        -1
    }
}