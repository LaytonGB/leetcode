use std::collections::VecDeque;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut res = VecDeque::new();
        for p in path.split('/') {
            match p {
                "" => {}
                "." => {}
                ".." => _ = res.pop_back(),
                s if s.chars().all(|c| c == '/') => {}
                s => res.push_back(s),
            }
        }

        let mut res = res.into_iter().fold(String::new(), |res, s| {
            res + "/" + s
        });

        if res.chars().next().is_none_or(|c| c != '/') {
            res.insert(0, '/');
        }
        while res.len() > 1 && res.chars().last().unwrap() == '/' {
            res.pop();
        }

        res
    }
}