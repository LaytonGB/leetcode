use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut a_set = HashSet::with_capacity(n);
        let mut b_set = HashSet::with_capacity(n);
        let mut res = Vec::with_capacity(n);

        for (a, b) in a.into_iter().zip(b.into_iter()) {
            a_set.insert(a);
            b_set.insert(b);

            res.push(a_set.intersection(&b_set).count() as i32);
        }

        res
    }
}