impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let n = blocks.len();
        let blocks = blocks.into_bytes();
        let mut w_count = 0;

        // init rolling min
        for i in 0..k {
            if blocks[i] == b'W' {
                w_count += 1;
            }
        }

        let mut rolling_min_changes = w_count;

        // iterate remaining blocks
        for i in k..n {
            if blocks[i - k] == b'W' {
                w_count -= 1;
            }
            if blocks[i] == b'W' {
                w_count += 1;
            }

            rolling_min_changes = rolling_min_changes.min(w_count);
        }

        rolling_min_changes
    }
}