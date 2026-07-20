fn to_char_idx(b: u8) -> u8 {
    b - b'0'
}

fn is_in_range(b: u8) -> bool {
    b >= 1 && b <= 26
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let mut memo = vec![-1; n + 1];
        memo[n] = 1;
        Self::r(s.as_bytes(), &mut memo, 0)
    }

    fn r(s: &[u8], memo: &mut [i32], i: usize) -> i32 {
        if memo[i] != -1 {
            memo[i]
        } else if s[i] == b'0' {
            memo[i] = 0;
            0
        } else {
            let (a, b) = (to_char_idx(s[i]), s.get(i + 1).map(|b| to_char_idx(*b)));
            // println!("a:{:?} | b:{:?}", a, b);

            if b.is_some_and(|b| is_in_range(a * 10 + b)) {
                memo[i] = Self::r(s, memo, i + 1) + Self::r(s, memo, i + 2);
            } else {
                memo[i] = Self::r(s, memo, i + 1);
            }

            memo[i]
        }
    }
}