impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut res = vec![];
        let mut lowest = i32::MAX;
        for i in 0..arr.len() - 1 {
            let diff = arr[i+1] - arr[i];
            if diff < lowest {
                res = vec![vec![arr[i], arr[i+1]]];
                lowest = diff;
            } else if diff == lowest {
                res.push(vec![arr[i], arr[i+1]]);
            }
        }
        res
    }
}