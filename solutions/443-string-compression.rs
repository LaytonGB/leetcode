impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (mut s, mut start_char_idx, mut f) = (0, 0, 1);
        let mut c = chars[0];
        while f < chars.len() {
            if chars[f] != c {
                chars[s] = c;
                s += 1;
                let diff = f - start_char_idx;
                if diff != 1 {
                    for c in diff.to_string().chars() {
                        if s >= chars.len() {
                            chars.push(c);
                        } else {
                            chars[s] = c;
                        }
                        s += 1; 
                    }
                }
                c = chars[f];
                start_char_idx = f;
            }
            f += 1;
        }
        chars[s] = c;
        s += 1;
        let diff = f - start_char_idx;
        if diff != 1 {
            for c in diff.to_string().chars() {
                if s >= chars.len() {
                    chars.push(c);
                } else {
                    chars[s] = c;
                }
                s += 1; 
            }
        }
        s as i32
    }
}