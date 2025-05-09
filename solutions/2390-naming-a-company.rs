use std::collections::HashSet;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let sets = ideas.iter()
            .fold(
                [(); 26].map(|_| HashSet::<&str>::new()),
                |mut a, n| {
                    a[(n.as_bytes()[0] - b'a') as usize].insert(&n[1..]);
                    a
            });
        // println!("{:?}", sets);
        let mut res = 0_i64;
        for (i, s1) in sets.iter().enumerate() {
            for s2 in sets.iter().skip(i + 1) {
                let d = s1.intersection(s2).collect::<HashSet<_>>().len();
                res += 2 * ((s1.len() - d) * (s2.len() - d)) as i64;
            }
        }
        // println!("{:?}", sets);
        res
    }
}