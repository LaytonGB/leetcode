impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut set = [-1_i32; 26];
        let mut last_set_start = 0_i32;
        let mut res = 1;
        for (i,c) in s.bytes().enumerate() {
            let idx = (c - b'a') as usize;
            if set[idx] >= last_set_start {
                res += 1;
                last_set_start = i as i32;
            }
            set[idx] = i as i32;
        }
        res
    }
}