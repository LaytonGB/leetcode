impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut no_stock, mut yes_stock) = (0, -prices[0] - fee); // (profit buy, profit sell)
        for i in 0..prices.len() {
            no_stock = no_stock.max(yes_stock + prices[i]);
            yes_stock = yes_stock.max(no_stock - prices[i] - fee);
        }
        // println!("{:?}", dp);
        no_stock
    }
}