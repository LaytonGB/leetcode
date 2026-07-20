use std::collections::BTreeMap;

#[derive(Debug)]
struct IdxStore {
    last: usize,
    sum: usize,
    count: usize,
}

impl IdxStore {
    pub fn update_from(&mut self, other: usize) {
        self.sum += self.count * self.last.abs_diff(other);
        self.count += 1;
        self.last = other;
    }
}

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();

        let pre = nums.iter().enumerate()
            .scan(BTreeMap::<i32, IdxStore>::new(), |mut state, (i, &x)| {
                let res = state.get(&x).map(|e| e.count * (i - e.last) + e.sum).unwrap_or(0);
                state.entry(x)
                    .and_modify(|e| e.update_from(i))
                    .or_insert(IdxStore{last: i, sum: 0, count: 1});
                Some(res)
            })
            .collect::<Vec<_>>();

        let post = nums.iter().enumerate().rev()
            .scan(BTreeMap::<i32, IdxStore>::new(), |mut state, (i, &x)| {
                let res = state.get(&x).map(|e| e.count * (e.last - i) + e.sum).unwrap_or(0);
                state.entry(x)
                    .and_modify(|e| e.update_from(i))
                    .or_insert(IdxStore{last: i, sum: 0, count: 1});
                Some(res)
            })
            .collect::<Vec<_>>();
            
        (0..n).map(|i| (pre[i] + post[n - i - 1]) as i64).collect()
    }
}