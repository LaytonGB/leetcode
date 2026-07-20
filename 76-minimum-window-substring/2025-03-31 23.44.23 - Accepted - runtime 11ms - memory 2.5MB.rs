use std::collections::VecDeque;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let target_chars = t.bytes().fold([0; 58], |mut res, b| {res[(b - b'A') as usize] += 1; res});
        let mut current_chars = [0; 58];
        let mut char_indeces = vec![VecDeque::new(); 58];
        let mut best = String::new();
        let mut best_len = usize::MAX;
        // println!("  {:?}", target_chars);

        s.bytes().enumerate().for_each(|(i, b)| {
            let c = (b - b'A') as usize;

            char_indeces[c].push_front(i);
            current_chars[c] += 1;

            if current_chars[c] > target_chars[c] {
                char_indeces[c].pop_back();
                current_chars[c] -= 1;
            }

            if current_chars.iter().zip(target_chars.iter()).all(|(a, b)| a >= b) {
                let (min, max) = char_indeces.iter()
                    .fold((usize::MAX, usize::MIN), |(min, max), q| {
                        (
                            min.min(*q.back().unwrap_or(&usize::MAX)),
                            max.max(*q.front().unwrap_or(&usize::MIN))
                        )
                    });

                if max - min < best_len {
                    best_len = max - min;
                    best = (&s[min..=max]).to_string();
                }
            }

            // println!("{} {:?}", b as char, current_chars);
        });

        best
    }
}