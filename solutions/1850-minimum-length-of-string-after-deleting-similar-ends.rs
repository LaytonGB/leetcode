impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.into_bytes();
        let (mut l, mut r) = (0, s.len() - 1);

        while l < r && s[l] == s[r] {
            let c = s[l];

            while l <= r && s[l] == c {
                l += 1;
            }

            while l <= r && s[r] == c {
                r -= 1;
            }
        }

        (r - l + 1) as i32
    }
}