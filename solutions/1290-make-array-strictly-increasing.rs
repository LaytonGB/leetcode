use std::collections::*;
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr2.sort_unstable();
        *arr1.iter()
            .fold(HashMap::from([(-1, 0)]), |mut dp, &n| {
                let mut tmp = HashMap::new();
                for k in dp.keys() {
                    if n > *k {
                        let dp_k = *dp.get(k).unwrap();
                        tmp.entry(n)
                            .and_modify(|e: &mut i32| *e = (*e).min(dp_k))
                            .or_insert(dp_k);
                    }
                    let loc = arr2.partition_point(|m| m <= k);
                    if loc < arr2.len() {
                        let dp_k = *dp.get(k).unwrap() + 1;
                        tmp.entry(arr2[loc])
                            .and_modify(|e: &mut i32| *e = (*e).min(dp_k))
                            .or_insert(dp_k);
                    }
                }
                tmp
            })
            .values()
            .min()
            .unwrap_or(&-1)
    }
}