impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        prices.iter()
            .enumerate()
            .map(|(i, p)| p - Self::get_discount(&prices, i))
            .collect()
    }

    fn get_discount(prices: &[i32], item: usize) -> i32 {
        *prices[item + 1..].iter()
            .find(|x| **x <= prices[item])
            .unwrap_or(&0)
    }
}