impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut res = 0;
        while num_bottles >= num_exchange {
            let d = num_bottles / num_exchange;
            let r = num_bottles % num_exchange;
            res += num_bottles - r;
            num_bottles = d + r;
        }
        res + num_bottles
    }
}