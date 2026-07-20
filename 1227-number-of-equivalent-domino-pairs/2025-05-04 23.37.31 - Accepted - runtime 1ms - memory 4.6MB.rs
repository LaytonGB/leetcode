impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut counts = [0; 81];
        dominoes.into_iter().for_each(|d| {
            let (a, b) = (d[0].min(d[1]), d[1].max(d[0]));
            counts[Self::hash(a, b)] += 1;
        });

        counts.into_iter().map(|x| x * (x - 1) / 2).sum()
    }

    fn hash(a: i32, b: i32) -> usize {
        (a * 9 + b) as usize - 10
    }
}