impl Solution {
    pub fn climb_stairs(mut n: i32) -> i32 {
        if n == 1 { return 1; }
        let (mut a, mut b) = (1, 2);
        for i in 2..n as usize {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}