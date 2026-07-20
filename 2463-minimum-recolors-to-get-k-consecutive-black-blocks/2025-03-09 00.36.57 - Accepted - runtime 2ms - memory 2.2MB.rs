impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let blocks = blocks.into_bytes();
        let mut init = blocks[0..k].iter().fold(0, |acc, b| acc + (*b == b'W') as i32);

        (0..blocks.len() - k)
            .map(|i| {
                println!("{} | {}", blocks[i] as char, blocks[i + k] as char);
                match (blocks[i], blocks[i + k]) {
                    (b'B', b'W') => 1,
                    (b'W', b'B') => -1,
                    _ => 0,
                }
            })
            .scan((init, init), |state, change| {
                // state.0 = float
                // state.1 = result
                if state.1 == 0 {
                    return None;
                }

                state.0 += change;
                
                Some(state.1.min(state.0))
            })
            .min()
            .unwrap_or(init)
    }
}