impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut res = Vec::with_capacity(num.len());
        for c in num.chars() {
            while !res.is_empty() && k > 0 && c < *res.last().unwrap() {
                res.pop();
                k -= 1;
            }
            res.push(c);
        }

        for _ in 0..k {
            res.pop();
        }

        let start = res.iter().position(|x| *x != '0');
        if let Some(start) = start {
            res[start..].into_iter().collect()
        } else {
            "0".to_string()
        }
    }
}