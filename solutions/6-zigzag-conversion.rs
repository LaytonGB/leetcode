impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let n = num_rows as usize;
        let mut strings = vec![String::new(); n];
        let (mut dir, mut idx) = (1, 0);
        for (i, c) in s.chars().enumerate() {
            strings[idx].push(c);
            if idx == 0 {
                dir = 1
            } else if idx == n - 1 {
                dir = !0;
            }
            idx += dir;
        }
        strings.concat()
    }
}