use std::collections::{HashMap, hash_map::Entry};

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut a = 0;
        let mut correct = vec![false; secret.len()];
        for (i, (s, g)) in secret.bytes().zip(guess.bytes()).enumerate() {
            if s == g {
                a += 1;
                correct[i] = true;
            }
        }

        let mut secret_chars = HashMap::new();
        for (i, s) in secret.bytes().enumerate() {
            if !correct[i] {
                secret_chars.entry(s)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }

        let mut b = 0;
        for (i, g) in guess.bytes().enumerate() {
            if !correct[i] {
                if let Entry::Occupied(mut s) = secret_chars.entry(g) {
                    b += 1;

                    let mut s_inner = s.get_mut();
                    if *s_inner == 1 {
                        s.remove();
                    } else {
                        *s_inner -= 1;
                    }
                }
            }
        }

        format!("{}A{}B", a, b)
    }
}