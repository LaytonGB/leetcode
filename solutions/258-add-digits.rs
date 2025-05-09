impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num.to_string();
        while num.len() > 1 {
            num = num
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .reduce(|a, b| a + b)
                .unwrap()
                .to_string();
        }
        num
            .chars()
            .take(1)
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
    }
}