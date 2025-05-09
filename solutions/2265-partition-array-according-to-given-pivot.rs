use std::cmp::Ordering as O;
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let (mut b, mut m, mut e) = (vec![], vec![], vec![]);

        nums.into_iter().for_each(|n| {
            match n.cmp(&pivot) {
                O::Less => b.push(n),
                O::Equal => m.push(n),
                O::Greater => e.push(n),
            }
        });

        [b, m, e].concat()
    }
}