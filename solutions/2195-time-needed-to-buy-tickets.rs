impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut res = 0_i32;
        for i in 0..tickets.len() {
            res += tickets[i].min(tickets[k] - (i > k) as i32);
        }
        res
    }
}