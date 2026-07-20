use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let target_chars = t.bytes()
            .fold(HashMap::new(), |mut res, b| {
                res.entry(b)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                res
            });

        let mut current_chars = HashMap::new();
        let mut char_indeces: HashMap<u8, VecDeque<usize>> = HashMap::new();
        let (mut best_min, mut best_max) = (usize::MAX, 0);
        let mut best_len = usize::MAX;
        // println!("  {:?}", target_chars);

        s.bytes().enumerate().for_each(|(i, b)| {
            if target_chars.get(&b).is_none() {
                return;
            }
            
            let indeces = char_indeces.entry(b)
                .and_modify(|e| e.push_front(i))
                .or_insert(VecDeque::from([i]));
            let count = current_chars.entry(b)
                .and_modify(|e| *e += 1)
                .or_insert(1);

            if target_chars.get(&b).is_some_and(|t| *count > *t) {
                indeces.pop_back();
                *count -= 1;
            }

            if target_chars.iter().all(|(b, t)| current_chars.get(b).is_some_and(|c| c >= t)) {
                let (min, max) = char_indeces.values()
                    .fold((usize::MAX, usize::MIN), |(min, max), q| {
                        (
                            min.min(*q.back().unwrap_or(&usize::MAX)),
                            max.max(*q.front().unwrap_or(&usize::MIN))
                        )
                    });

                if max.checked_sub(min).unwrap_or(usize::MAX) < best_len {
                    best_len = max - min;
                    (best_min, best_max) = (min, max);
                }
            }

            // println!("{} {:?}", b as char, current_chars);
        });

        if best_min == usize::MAX {
            String::new()
        } else {
            s[best_min..=best_max].to_string()
        }
    }
}