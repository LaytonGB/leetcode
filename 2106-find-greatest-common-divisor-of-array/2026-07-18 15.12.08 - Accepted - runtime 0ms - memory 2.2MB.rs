impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (min, max) = nums.into_iter()
            .fold((i32::MAX, 0), |(min, max), x| {
                (min.min(x), max.max(x))
            });
        
        Self::gcd(min, max)
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 {
            let a_prev = a;
            a = b % a;
            b = a_prev;
        }
        b
    }
}