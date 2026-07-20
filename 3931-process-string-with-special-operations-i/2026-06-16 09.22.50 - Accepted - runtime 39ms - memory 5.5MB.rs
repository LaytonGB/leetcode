use std::collections::VecDeque;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut res = VecDeque::new();
        let mut is_rev = false;
        for c in s.chars() {
            match c {
                '*' => Self::pop(&mut res, is_rev),
                '#' => Self::duplicate(&mut res, is_rev),
                '%' => is_rev = !is_rev,
                _ => Self::append(&mut res, is_rev, c),
            }
        }
        match is_rev {
            false => res.into_iter().collect(),
            true => res.into_iter().rev().collect(),
        }
    }

    fn append(res: &mut VecDeque<char>, is_rev: bool, c: char) {
        match is_rev {
            false => res.push_back(c),
            true => res.push_front(c),
        }
    }

    fn pop(res: &mut VecDeque<char>, is_rev: bool) {
        match is_rev {
            false => { res.pop_back(); }
            true => { res.pop_front(); }
        }
    }

    fn duplicate(res: &mut VecDeque<char>, is_rev: bool) {
        let mut dup: VecDeque<char> = res.iter().copied().collect();
        res.append(&mut dup);
    }
}