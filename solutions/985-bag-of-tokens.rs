use std::collections::VecDeque;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, power: i32) -> i32 {
        let n = tokens.len();

        if n == 0 {
            return 0;
        }
        
        tokens.sort_unstable();
        let tokens = tokens;

        if tokens[0] > power {
            return 0;
        }

        let mut best_score = 0;
        for i in 0..n {
            let mut score = 0;
            let mut power = power;
            for j in 0..=i {
                score += 1;
                power -= tokens[j];
            }
            for j in (i+1..n).rev() {
                if power >= 0 || score < 0 {
                    break;
                }

                score -= 1;
                power += tokens[j];
            }

            if power >= 0 && score > best_score {
                best_score = score;
            }
        }

        best_score
    }
}
