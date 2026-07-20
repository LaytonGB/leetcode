use std::collections::BTreeSet;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut lookup = vec![None; 26];
        let mut group = vec![None; 26];
        let mut set_count = 0;
        for (b1, b2) in s1.bytes().zip(s2.bytes()) {
            if b1 == b2 {
                continue;
            }

            match (lookup[(b1 - b'a') as usize], lookup[(b2 - b'a') as usize]) {
                (None, None) => {
                    lookup[(b1 - b'a') as usize] = Some(set_count);
                    lookup[(b2 - b'a') as usize] = Some(set_count);
                    group[set_count] = Some(BTreeSet::from([b1, b2]));
                    
                    while matches!(group.get(set_count), Some(Some(_))) {
                        set_count += 1;
                    }
                }
                (Some(set_idx), None) | (None, Some(set_idx)) => {
                    lookup[(b1 - b'a') as usize] = Some(set_idx);
                    lookup[(b2 - b'a') as usize] = Some(set_idx);
                    group[set_idx].as_mut().unwrap().insert(b1);
                    group[set_idx].as_mut().unwrap().insert(b2);
                }
                (Some(set_i), Some(set_j)) => {
                    if set_i == set_j {
                        continue;
                    }
                    
                    let from_idx = set_i.max(set_j);
                    let to_idx = set_i.min(set_j);

                    let mut from = group[from_idx].take().unwrap();
                    let mut to = group[to_idx].as_mut().unwrap();
                    for b in from.into_iter() {
                        to.insert(b);
                        lookup[(b - b'a') as usize] = Some(to_idx);
                    }

                    set_count = from_idx;
                }
            }
        }

        let mut res = String::new();
        for b in base_str.bytes() {
            if let Some(set_idx) = lookup[(b - b'a') as usize] {
                let set = group[set_idx].as_ref().unwrap();
                res.push(*set.first().unwrap() as char);
            } else {
                res.push(b as char);
            }
        }

        res
    }
}