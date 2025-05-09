use std::collections::*;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = nums.iter().fold(HashMap::with_capacity(nums.len()), |mut m, n| {
            *m.entry(n).or_insert(0) += 1;
            m
        })
            .iter().map(|(k,v)| (*v, **k)).collect::<Vec<(usize, i32)>>();
        res.sort();
        res[res.len() - k as usize ..].iter().map(|(v,k)| *k).collect()
    }
}