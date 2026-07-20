impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut nums = vec![];
        while n > 0 {
            let d = n % 10;
            if d != 0 {
                nums.push(d);
            }
            n /= 10;
        }

        let mut x = 0;
        let mut sum = 0;
        while let Some(d) = nums.pop() {
            x = x * 10 + d;
            sum += d;
        }

        x as i64 * sum as i64
    }
}