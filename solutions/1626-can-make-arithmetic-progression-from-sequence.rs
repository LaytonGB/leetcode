impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let mut diff = arr[1] - arr[0];
        for w in arr.windows(2).skip(1) {
            let next_diff = w[1] - w[0];
            if diff != next_diff { return false; }
            diff = next_diff;
        }
        true
    }
}