impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let (mut l, mut h) = (0, candies.iter().max().unwrap() + 1);

        while l < h {
            let m = (h + l + 1) / 2;
            
            let n = candies.iter()
                .fold(0_i64, |sum, c| {
                    sum + (c / m) as i64
                });
            
            if n >= k {
                l = m;
            } else {
                h = m - 1;
            }
        }

        l
    }
}