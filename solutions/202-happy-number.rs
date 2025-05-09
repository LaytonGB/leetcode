impl Solution {
    pub fn squareSum(int: &i32) -> i32 {
        let mut temp;
        let mut out = 0_i32;
        let mut n = int.to_owned();
        while n > 0 {
            temp = n % 10;
            out += temp * temp;
            n /= 10;
        }
        out
    }
    
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Self::squareSum(&slow);
            fast = Self::squareSum(&fast);
            fast = Self::squareSum(&fast);
            if fast == 1 || slow == fast {
                break;
            }
        }
        if fast == 1 {
            return true;
        }
        false
    }
}