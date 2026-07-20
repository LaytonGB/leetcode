impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;

        for i in 0..s.len() {
            if s.len() - i <= res {
                break;
            }

            let mut counts = [0; 26];
            let mut max_count = 0;

            for j in i..s.len() {
                let c = s[j];
                let idx = (c - b'a') as usize;
                counts[idx] += 1;
                max_count = max_count.max(counts[idx]);

                if counts.iter().all(|&c| c == 0 || c == max_count) && j - i + 1 > res {
                    res = j - i + 1;
                }
            }
        }

        res as i32
    }
}