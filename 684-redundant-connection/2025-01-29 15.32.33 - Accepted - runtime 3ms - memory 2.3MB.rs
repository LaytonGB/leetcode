use std::collections::HashMap;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::with_capacity(edges.len() - 1);
        
        edges.iter()
            .find(|pair| {
                let [a, b] = &pair[..] else {
                    unreachable!();
                };

                let a_group = *map.entry(a).or_insert(a);
                let b_group = *map.entry(b).or_insert(b);

                if a_group == b_group {
                    return true;
                }

                map.iter_mut()
                    .for_each(|(_, val)| {
                        if *val == b_group {
                            *val = a_group;
                        }
                    });

                return false;
            })
            .unwrap()
            .clone()
    }
}