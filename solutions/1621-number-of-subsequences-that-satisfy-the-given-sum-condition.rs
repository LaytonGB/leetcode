const MOD: i32 = 10_i32.pow(9) + 7;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        // variables
        let mut res = 0; // output
        let (mut l, mut h) = (0, nums.len() - 1); // search bounds
        let mut subs = Vec::with_capacity(nums.len()); // subsequence counts

        // calc number of subsequences at each position (binary exponentiation)
        subs.push(1);
        for i in 1..=nums.len() {
            subs.push((subs.last().unwrap() << 1) % MOD);
        }

        nums.sort();

        // within bounds for both low and high
        while l <= h {
            if nums[l] + nums[h] > target {
                if h == 0 { break; } // break if going out of bounds
                h -= 1;
            } else {
                res = (res + subs[h - l]) % MOD;
                l += 1;
            }
        }

        res
    }
}