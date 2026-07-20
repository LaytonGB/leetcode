use std::cmp::Ordering as O;

impl Solution {
    pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let n = nums.len();

        let mut lt = Vec::with_capacity(n);
        let mut eq = Vec::with_capacity(n);
        let mut gt = Vec::with_capacity(n);
        for x in nums {
            match x.cmp(&pivot) {
                O::Less => lt.push(x),
                O::Equal => eq.push(x),
                O::Greater => gt.push(x),
            }
        }

        lt.extend(eq);
        lt.extend(gt);

        lt
    }
}