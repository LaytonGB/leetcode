use std::cmp::{min, max};

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let [mut missing_lower, mut missing_upper, mut missing_digit] = [true; 3];
        for c in password.chars() {
            if c.is_ascii_lowercase() { missing_lower = false; }
            else if c.is_ascii_uppercase() { missing_upper = false; }
            else if c.is_digit(10) { missing_digit = false; }
        }
        let missing = missing_lower as i32 + missing_upper as i32 + missing_digit as i32;

        let [mut d1, mut d2, mut r] = [0; 3];
        let p = password.as_bytes();
        let mut i = 2;
        while i < p.len() {
            if p[i-2] == p[i-1] && p[i-1] == p[i] {
                let mut l = 3;
                while i + 1 < p.len() && p[i+1] == p[i] {
                    i += 1;
                    l += 1;
                }
                if l % 3 == 0 { d1 += 1 }
                else if l % 3 == 1 { d2 += 1 }
                r += l / 3;
            }
            i += 1;
        }

        if p.len() < 6 {
            max(missing, 6 - p.len() as i32)
        } else if p.len() <= 20 {
            max(missing, r)
        } else {
            let del = (p.len() - 20) as i32;
            r -= min(d1, del);
            if del - d1 > 0 {
                r -= min((del - d1) / 2, d2);
            }
            if del - d1 - 2 * d2 > 0 {
                r -= (del - d1 - 2 * d2) / 3;
            }
            del + max(missing, r)
        }
    }
}