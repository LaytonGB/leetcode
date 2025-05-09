// https://leetcode.com/problems/find-the-winner-of-the-circular-game/solutions/5438775/explanations-no-one-will-give-you-3-detailed-approaches-extremely-simple-and-effective
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut res = 0_i32;
        for i in 2..=n {
            res = (res + k) % i;
        }
        res + 1
    }
}