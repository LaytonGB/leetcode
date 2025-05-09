impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as i64;
        
        let (mut l, mut h) = (1, *ranks.iter().min().unwrap() as i64 * cars * cars);
        while l < h {
            let m = (l + h) / 2;

            if (
                ranks.iter()
                    .fold(0, |cars_done, r| {
                        cars_done + (m / *r as i64).isqrt()
                    })
                    >= cars
            ) {
                h = m;
            } else {
                l = m + 1;
            }
        }

        l
    }
}