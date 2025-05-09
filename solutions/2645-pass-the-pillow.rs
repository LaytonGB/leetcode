impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        (1..n).chain((2..=n).rev())
            .nth((time % (2 * n - 2)) as usize)
            .unwrap()
    }
}