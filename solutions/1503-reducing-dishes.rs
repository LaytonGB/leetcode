impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        let n = satisfaction.len();
        satisfaction.sort_unstable();
        let mut i = 1;
        let mut res = 0;
        let mut next = satisfaction[n - i];
        loop {
            if next >= res {
                res = next;
            } else {
                break;
            }
            i += 1;
            if i > n {
                break;
            }
            for j in 1 ..= i {
                next += satisfaction[n - j];
            }
        }
        res
    }
}