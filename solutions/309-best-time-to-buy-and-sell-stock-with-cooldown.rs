impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // solution from https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/solutions/75931/easiest-java-solution-with-explanations/

        if prices.len() == 1 { return 0; }
    
        // start as though we've bought prices[0]
        let (mut b0, mut b1) = (-prices[0], -prices[0]);
        // start as though we've sold nothing
        let (mut s0, mut s1, mut s2) = (0, 0, 0);

        for i in 0..prices.len() {
            // stay with current debt or sell current price based on
            // claimed profit from selling 3 prices ago
            b0 = b1.max(s2 - prices[i]);
            // stay with current profit or sell current price
            s0 = s1.max(b1 + prices[i]);
            // shift all buys and sells from n to n+1
            b1 = b0; s2 = s1; s1 = s0;
        }
        s0
    }
}