// https://leetcode.com/problems/candy/solutions/2234434/c-o-n-time-o-1-space-full-explanation
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        
        let mut res = ratings.len();
        let mut i = 1;
        while i < n {
            if ratings[i - 1] == ratings[i] {
                i += 1;
                continue;
            }

            let mut peak = 0;
            while ratings[i - 1] < ratings[i] {
                peak += 1;
                res += peak;
                i += 1;
                
                if i == n {
                    return res as i32;
                }
            }

            let mut trough = 0;
            while i < n && ratings[i - 1] > ratings[i] {
                trough += 1;
                res += trough;
                i += 1;
            }

            res -= peak.min(trough);
        }

        res as i32
    }
}