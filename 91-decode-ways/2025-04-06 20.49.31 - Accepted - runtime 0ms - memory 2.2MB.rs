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

        let (mut x, mut y, mut z) = (0, 1, 0);

        for i in (0..n).rev() {
            if s[i] == b'0' {
                x = 0;
            } else {
                x = y;

                if i < n - 1 {
                    let (a, b) = (to_char_idx(s[i]), to_char_idx(s[i + 1]));

                    if is_in_range(a * 10 + b) {
                        x += z;
                    }
                }
            }

            z = y;
            y = x;
        }

        // println!("x:{} y:{} z:{}", x, y, z);
        x
    }
}