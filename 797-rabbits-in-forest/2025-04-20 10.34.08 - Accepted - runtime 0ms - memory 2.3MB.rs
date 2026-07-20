use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for a in answers.into_iter() {
            map.entry(a + 1)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        map.into_iter()
            .map(|(k, v)| ((v + k - 1) / k) * k)
            .sum()
    }
}