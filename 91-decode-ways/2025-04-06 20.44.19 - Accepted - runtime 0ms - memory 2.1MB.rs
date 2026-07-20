fn to_char_idx(b: u8) -> u8 {
    b - b'0'
}

fn is_in_range(b: u8) -> bool {
    b >= 1 && b <= 26
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();

        let mut dp = vec![0; n + 1];
        dp[n] = 1;

        for i in (0..n).rev() {
            if s[i] == b'0' {
                dp[i] = 0;
            } else {
                dp[i] = dp[i + 1];

                if i < n - 1 {
                    let (a, b) = (to_char_idx(s[i]), to_char_idx(s[i + 1]));

                    if is_in_range(a * 10 + b) {
                        dp[i] += dp[i + 2];
                    }
                }
            }
        }

        // println!("{:?}", dp);
        dp[0]
    }
}