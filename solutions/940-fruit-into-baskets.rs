use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        let (mut i, mut j) = (0, 0);
        while i < fruits.len() {
            *m.entry(fruits[i]).or_insert(0) += 1;
            if m.len() <= 2 {
                res = res.max(i - j + 1);
            } else {
                *m.get_mut(&fruits[j]).unwrap() -= 1;
                if *m.get(&fruits[j]).unwrap() == 0 {
                    m.remove(&fruits[j]);
                }
                j += 1;
            }
            i += 1;
        }
        res as i32
    }
}