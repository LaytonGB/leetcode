impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut res = String::with_capacity(word.len() * 2);
        let mut chars = word.chars();
        let mut last = chars.next().unwrap();
        let mut n = 1;
        for c in chars {
            if last == c {
                if n == 9 {
                    res.push_str(&n.to_string());
                    res.push(last);
                    n = 1;
                } else {
                    n += 1;
                }
            } else {
                res.push_str(&n.to_string());
                res.push(last);
                n = 1;
                last = c;
            }
        }
        res.push_str(&n.to_string());
        res.push(last);
        res
    }
}