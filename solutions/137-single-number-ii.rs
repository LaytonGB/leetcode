use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut once = HashSet::new();
        let mut twice_or_more = HashSet::new();
        for n in nums.iter() {
            if !twice_or_more.contains(n) {
                if once.contains(n) {
                    once.remove(n);
                    twice_or_more.insert(*n);
                } else {
                    once.insert(*n);
                }
            }
        }
        *once.iter().next().unwrap()
    }
}