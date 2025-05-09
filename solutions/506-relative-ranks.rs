impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        
        let mut sorted = score.clone();
        sorted.sort_unstable();

        let mut res = Vec::with_capacity(score.len());
        for s in score.iter() {
            let i = n - sorted.binary_search(s).unwrap();
            res.push(
                match i {
                    1 => "Gold Medal".to_string(),
                    2 => "Silver Medal".to_string(),
                    3 => "Bronze Medal".to_string(),
                    _ => i.to_string(),
                }
            );
        }

        res
    }
}