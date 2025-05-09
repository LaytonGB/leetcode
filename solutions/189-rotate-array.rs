impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n == 1 { return; }
        let k = n - (k as usize % n);
        if k == 0 { return; }
        let mut moved = 0;
        let mut last_pos: usize;
        let mut offset = 0;
        let mut temp: i32;
        let mut i;
        while moved < n {
            i = offset;
            last_pos = n - k + offset;
            temp = nums[i];
            while i % n != last_pos {
                nums[i % n] = nums[(i + k) % n];
                moved += 1;
                i += k;
            }
            nums[last_pos] = temp;
            moved += 1;
            offset += 1;
        }
    }
}