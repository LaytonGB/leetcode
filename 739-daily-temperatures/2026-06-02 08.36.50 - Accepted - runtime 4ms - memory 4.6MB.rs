impl Solution {
    pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temps.len()];
        let mut stack = Vec::with_capacity(temps.len());
        for (i, &t) in temps.iter().enumerate() {
            while let Some(&j) = stack.last() {
                if t > temps[j] {
                    res[j] = (i - j) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        res
    }
}