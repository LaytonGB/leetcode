fn sum(candidates: &Vec<i32>, target: i32, out: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>, idx: usize) {
    if target < 0 {
        return;
    }
    if target == 0 {
        out.push(curr.to_vec());
        return;
    }
    for i in idx..candidates.len() {
        if target - candidates[i] < 0 {
            break;
        }
        curr.push(candidates[i]);
        sum(candidates, target - candidates[i], out, curr, i);
        curr.pop();
    }
}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        candidates.dedup();
        let mut out: Vec<Vec<i32>> = Vec::new();
        sum(&candidates, target, &mut out, &mut Vec::new(), 0);
        out
    }
}