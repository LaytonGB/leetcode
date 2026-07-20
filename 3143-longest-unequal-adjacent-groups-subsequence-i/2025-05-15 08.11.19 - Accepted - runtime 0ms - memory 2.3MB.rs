impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut sub_seq_len = [0, 0];
        for i in 0..words.len() {
            sub_seq_len[groups[i] as usize] = sub_seq_len[((groups[i] as usize + 1) % 2)] + 1;
        }

        let max_pos = if sub_seq_len[0] >= sub_seq_len[1] {
            0
        } else {
            1
        };
        let mut g = ((sub_seq_len[max_pos] + max_pos + 1) % 2) as i32;
        let mut res = Vec::with_capacity(sub_seq_len[g as usize]);
        for (i, w) in words.into_iter().enumerate() {
            if groups[i] == g {
                res.push(w);
                g = (g + 1) % 2;
            }
        }

        res
    }
}