impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        
        let (mut a, mut b) = (arr[0] & 1 == 1, arr[1] & 1 == 1);
        for c in arr[2..].iter().map(|x| x & 1 == 1) {
            if a && b && c {
                return true;
            }

            a = b;
            b = c;
        }
        false
    }
}