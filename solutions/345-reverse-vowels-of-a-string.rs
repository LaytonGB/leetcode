impl Solution {
    pub fn reverse_vowels(mut st: String) -> String {
        if st.len() < 2 {
            return st;
        }
        
        let mut s: Vec<char> = st.chars().collect();
        let vowels = std::collections::HashSet::from(['a','e','i','o','u','A','E','I','O','U']);
        let (mut l, mut h) = (0, st.len() - 1);
        let mut t;
        loop {
            while l < h && !vowels.contains(&s[l]) {
                l += 1;
            }
            while l < h && !vowels.contains(&s[h]) {
                h -= 1;
            }
            if l >= h {
                break;
            }
            t = s[l];
            s[l] = s[h];
            s[h] = t;
            l += 1;
            h -= 1;
        }
        s.iter().collect()
    }
}