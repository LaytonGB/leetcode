impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut streaks = vec![];
        let mut s = 0;
        let mut highest_solo_streak = 0;
        for w in nums.windows(2) {
            if s > highest_solo_streak {
                highest_solo_streak = s;
            }
            if w[0] < w[1] {
                s += 1;
            } else {
                streaks.push(s);
                s = 0;
            }
        }
        if s > 0 {
            if s > highest_solo_streak {
                highest_solo_streak = s;
            }
            streaks.push(s);
        }

        let mut highest_low_from_pairs = 0;
        for w in streaks.windows(2) {
            let b = w[0].min(w[1]);
            if b > highest_low_from_pairs {
                highest_low_from_pairs = b;
            }
        }

        return (highest_low_from_pairs + 1).max((highest_solo_streak + 1) / 2)
    }
}