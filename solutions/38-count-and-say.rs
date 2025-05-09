fn get_s(s: String) -> String {
    let mut res: String = String::new();
    let mut last = None;
    let mut count = 0;
    for c in s.chars() {
        if last.is_some_and(|l| l == c) {
            count += 1;
        } else {
            if let Some(l) = last {
                res.push_str(&count.to_string());
                res.push(l);
            }
            
            last = Some(c);
            count = 1;
        }
    }

    if let Some(l) = last {
        res.push_str(&count.to_string());
        res.push(l);
    }
    
    res
}

impl Solution {
    pub fn count_and_say(mut n: i32) -> String {
        let mut s = "1".to_string();

        while n > 1 {
            n -= 1;

            s = get_s(s);
        }

        s
    }
}