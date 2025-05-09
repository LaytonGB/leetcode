impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = Vec::new();
        let mut to_include = vec![true; s.len()];
        let mut res = String::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => stack.push(i),
                ')' => {
                    if stack.pop().is_none() {
                        to_include[i] = false;
                    }
                }
                _ => ()
            }
        }

        for i in stack.iter() {
            to_include[*i] = false;
        }

        for (i, c) in s.chars().enumerate() {
            if to_include[i] {
                res.push(c);
            }
        }

        res
    }
}