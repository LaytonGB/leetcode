impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut temp = Vec::new();
        let mut res = Vec::new();
        Self::backtrack(&mut res, &mut temp, &s[..]);
        res
    }

    fn backtrack(res: &mut Vec<Vec<String>>, temp: &mut Vec<String>, s: &str) {
        if s.is_empty() {
            res.push(temp.clone());
        } else {
            for i in 0..s.len() {
                if Self::is_palindrome(s, i) {
                    temp.push(s[..=i].to_string());
                    Self::backtrack(res, temp, &s[i+1..]);
                    temp.pop();
                }
            }
        }
    }

    fn is_palindrome(s: &str, mut r: usize) -> bool {
        let mut l = 0;
        while l < r {
            if s[l..l+1] != s[r..r+1] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}