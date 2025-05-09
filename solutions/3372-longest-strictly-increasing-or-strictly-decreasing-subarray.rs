use std::cmp::Ordering as O;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Equal,
    Increasing,
    Decreasing,
}

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut s = State::Equal;
        let mut x = nums[0];
        let mut curr = 1;
        let mut res = 1;
        for y in &nums[1..] {
            match (s, x.cmp(y)) {
                (State::Equal, O::Less) => {
                    s = State::Increasing;
                    curr = 2;
                    res = res.max(curr);
                }
                (State::Equal, O::Equal) => {}
                (State::Equal, O::Greater) => {
                    s = State::Decreasing;
                    curr = 2;
                    res = res.max(curr);
                }
                (State::Increasing, O::Less) => {
                    curr += 1;
                    res = res.max(curr);
                }
                (State::Increasing, O::Equal) => {
                    s = State::Equal;
                }
                (State::Increasing, O::Greater) => {
                    s = State::Decreasing;
                    curr = 2;
                }
                (State::Decreasing, O::Less) => {
                    s = State::Increasing;
                    curr = 2;
                }
                (State::Decreasing, O::Equal) => {
                    s = State::Equal;
                }
                (State::Decreasing, O::Greater) => {
                    curr += 1;
                    res = res.max(curr);
                }
            };
            x = *y;
        }
        res
    }
}