// https://leetcode.com/problems/maximum-score-words-formed-by-letters/solutions/425129/java-backtrack-similar-to-78-subsets-1ms-beats-100
// code modified to add a bit of early termination

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut lc = letters.iter()
            .fold([0; 26], |mut c, &l| {c[(l as u8 - b'a') as usize] += 1; c});

        Self::backtrack(&words, &mut lc, &score, 0)
    }

    fn backtrack(words: &[String], lc: &mut [i32; 26], score: &[i32], start: usize) -> i32 {
        let mut max = 0;
        'outer: for i in start..words.len() {
            let mut res = 0;

            for (j, b) in words[i].bytes().enumerate() {
                lc[(b - b'a') as usize] -= 1;
                res += score[(b - b'a') as usize];

                if lc[(b - b'a') as usize] < 0 {
                    for b in words[i].bytes().take(j + 1) {
                        lc[(b - b'a') as usize] += 1;
                    }
                    continue 'outer;
                }
            }

            res += Self::backtrack(words, lc, score, i + 1);
            max = max.max(res);

            for b in words[i].bytes() {
                lc[(b - b'a') as usize] += 1;
            }
        }

        max
    }
}