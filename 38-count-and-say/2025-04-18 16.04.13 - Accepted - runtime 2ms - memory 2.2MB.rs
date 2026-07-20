fn get_parts(s: &str) -> Vec<(usize, char)> {
    let mut parts: Vec<(usize, char)> = Vec::with_capacity(s.len());
    let mut last = None;
    for c in s.chars() {
        if last.is_some_and(|l| l == c) {
            parts.last_mut().unwrap().0 += 1;
        } else {
            parts.push((1, c));
        }

        last = Some(c);
    }
    
    parts
}

impl Solution {
    pub fn count_and_say(mut n: i32) -> String {
        let mut s = "1".to_string();

        while n > 1 {
            n -= 1;

            s = get_parts(&s).into_iter()
                .map(|p| format!("{}{}", p.0, p.1))
                .collect();
        }

        s
    }
}