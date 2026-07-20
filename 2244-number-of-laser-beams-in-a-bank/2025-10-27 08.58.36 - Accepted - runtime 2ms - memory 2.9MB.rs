impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.iter()
            .map(|s| s.bytes().filter(|b| *b == b'1').count())
            .filter(|c| *c > 0)
            .collect::<Vec<usize>>()
            .windows(2)
            .fold((0, 0), |(sum, prev), w| (sum + w[0] * w[1], w[1]))
            .0 as i32
    }
}