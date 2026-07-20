#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Domino {
    a: i32,
    b: i32,
}

impl Domino {
    pub fn new(d: Vec<i32>) -> Self {
        Self {
            a: d[0].min(d[1]),
            b: d[0].max(d[1]),
        }
    }
}

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut counts = std::collections::HashMap::with_capacity(81);
        dominoes.into_iter().for_each(|d| {
            let d = Domino::new(d);
            counts.entry(d)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });

        counts.values().map(|v| *v * (*v - 1) / 2).sum()
    }
}