#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Unseen,
    LowerSeen,
    UpperSeen,
    Invalid,
}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut states = [State::Unseen; 26];

        for b in word.bytes() {
            if b < 97 {
                let i = (b - b'A') as usize;
                states[i] = match states[i] {
                    State::Unseen | State::Invalid => State::Invalid,
                    State::LowerSeen | State::UpperSeen => State::UpperSeen,
                }
            } else {
                let i = (b - b'a') as usize;
                states[i] = match states[i] {
                    State::UpperSeen | State::Invalid => State::Invalid,
                    State::Unseen | State::LowerSeen => State::LowerSeen,
                }
            }
        }

        let mut res = 0;
        for s in states {
            if s == State::UpperSeen {
                res += 1;
            }
        }

        res
    }
}