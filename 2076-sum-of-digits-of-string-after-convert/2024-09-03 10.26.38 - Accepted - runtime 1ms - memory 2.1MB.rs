impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        fn char_map(c: char) -> String {
            (c as u8 - b'a' + 1).to_string()
        }

        fn char_sum(acc: i32, n: char) -> i32 {
            acc + (n as u8 - b'0') as i32
        }

        let mut n = s.chars()
            .map(char_map)
            .collect::<String>();

        for _ in 0..k {
            n = n.chars().fold(0, char_sum).to_string();
        }

        n.parse::<i32>().unwrap()
    }
}