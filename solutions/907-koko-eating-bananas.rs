impl Solution {
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        let (mut l, mut r) = (1, *piles.iter().max().unwrap());
        while l < r {
            let m = l + ((r - l) / 2);
            let time_req = piles.iter().fold(0, |acc, n| {
                acc + ((n + m - 1) / m)
            });
            if time_req > h {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}