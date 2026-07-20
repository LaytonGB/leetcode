impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let n = blocks.len();
        let blocks = blocks.into_bytes();
        
        let mut min_changes = i32::MAX;
        let mut changes_needed = 0;

        // init
        for i in 0..k as usize {
            if blocks[i] == b'W' {
                changes_needed += 1;
            }
        }

        min_changes = changes_needed;

        // test
        for i in k as usize..n {
            if blocks[i - k] == b'W' {
                changes_needed -= 1;
            }
            if blocks[i] == b'W' {
                changes_needed += 1;
            }

            min_changes = min_changes.min(changes_needed);
        }

        min_changes
    }
}