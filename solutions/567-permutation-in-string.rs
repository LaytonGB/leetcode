use std::collections::VecDeque;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut a = [0_usize; 26];
        for c in s1.into_bytes() {
            a[(c - b'a') as usize] += 1;
        }
        let mut q = VecDeque::new();
        let mut i;
        for c in s2.into_bytes() {
            i = (c - b'a') as usize;
            if a[i] > 0 {
                a[i] -= 1;
                q.push_back(i);
                if a[i] == 0 && a.iter().all(|&n| n == 0) {
                    return true;
                }
            } else {
                while let Some(j) = q.pop_front() {
                    a[j as usize] += 1;
                    if a[i] > 0 { 
                        a[i] -= 1;
                        q.push_back(i);
                        break;
                    }
                }
            }
        }
        false
    }
}