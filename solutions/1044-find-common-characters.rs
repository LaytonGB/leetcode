impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        words.into_iter()
            .fold([u8::MAX; 26], |mut res, w| {
                let counts = w.bytes().fold([0; 26], |mut res, b| {
                    res[(b - b'a') as usize] += 1;
                    res
                });
                res.iter_mut().zip(counts.iter()).for_each(|(c1, c2)| *c1 = (*c1).min(*c2));
                res
            })
            .into_iter()
            .enumerate()
            .fold(Vec::with_capacity(100), |mut res, (i, c)| {
                (0..c).for_each(|_| res.push(((i as u8 + b'a') as char).to_string()));
                res
            })
    }
}