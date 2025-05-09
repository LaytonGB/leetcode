impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        
        let mut odds = 0;
        for x in arr {
            if x % 2 == 1 {
                odds += 1;
            } else {
                odds = 0;
            }

            if odds == 3 {
                return true;
            }
        }

        false
    }
}
