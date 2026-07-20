impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut idxs = (0..names.len()).collect::<Vec<_>>();
        idxs.sort_unstable_by_key(|&i| -heights[i]);
        idxs.iter().map(|&i| names[i].clone()).collect()
    }
}