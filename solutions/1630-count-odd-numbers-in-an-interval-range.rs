impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let len = high + 1 - low;
        if len % 2 == 1 && low % 2 == 1 {
            len / 2 + 1
        } else {
            len / 2
        }
    }
}