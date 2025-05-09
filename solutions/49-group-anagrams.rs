use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut counts: HashMap<[i32; 26], Vec<String>> = HashMap::with_capacity(strs.len());
        for s in strs.into_iter() {
            let mut count = [0; 26];
            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }
            if let Some(v) = counts.get_mut(&count) {
                v.push(s);
            } else {
                counts.insert(count, vec![s]);
            }
        }

        counts.into_iter().map(|x| x.1).collect()
    }
}