impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n).map(|i| {
            let fizz = i % 3 == 0;
            let buzz = i % 5 == 0;
            if fizz && buzz {
                "FizzBuzz".to_string()
            } else if fizz {
                "Fizz".to_string()
            } else if buzz {
                "Buzz".to_string()
            } else {
                i.to_string()
            }
        }).collect()
    }
}