impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest = prices[0];
        let mut res = 0;
        for i in 1..prices.len() {
            let p = prices[i];
            lowest = lowest.min(p);
            res = res.max(p - lowest);
        }
        res
    }
}