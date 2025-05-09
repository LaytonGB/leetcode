impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut count = [0_usize; 1024];
        count[0] = 1;
        
        let mut mask = 0_usize;
        let mut res = 0;

        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            mask ^= 1 << idx;
            res += count[mask];

            for i in 0..10 {
                let search_idx = mask ^ (1 << i);
                res += count[search_idx];
            }

            count[mask] += 1;
        }

        res as i64
    }
}
