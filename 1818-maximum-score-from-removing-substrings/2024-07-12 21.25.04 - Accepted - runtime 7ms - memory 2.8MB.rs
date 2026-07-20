impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let n = s.len();
        let mut res = 0;

        let mut s1 = Vec::with_capacity(n);
        let mut s2 = Vec::with_capacity(n);

        if x >= y {
            for c in s.chars() {
                match (s1.last(), c) {
                    (Some('a'), 'b') => {
                        res += x;
                        s1.pop();
                    }
                    _ => s1.push(c),
                }
            }

            for c in s1.into_iter() {
                match (s2.last(), c) {
                    (Some('b'), 'a') => {
                        res += y;
                        s2.pop();
                    }
                    _ => s2.push(c),
                }
            }
        } else {
            for c in s.chars() {
                match (s1.last(), c) {
                    (Some('b'), 'a') => {
                        res += y;
                        s1.pop();
                    }
                    _ => s1.push(c),
                }
            }

            for c in s1.into_iter() {
                match (s2.last(), c) {
                    (Some('a'), 'b') => {
                        res += x;
                        s2.pop();
                    }
                    _ => s2.push(c),
                }
            }
        }

        res
    }
}