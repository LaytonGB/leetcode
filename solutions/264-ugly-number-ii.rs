struct UglyNumbers {
    sorted_set: Vec<i32>,
}

impl UglyNumbers {
    fn nth(mut n: usize) -> i32 {
        let mut un = Self {
            sorted_set: vec![1],
        };
        for _ in 0..n {
            un.next();
        }
        un.next().unwrap()
    }
}

impl Iterator for UglyNumbers {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let mut h = None;
        if let [head, rest @ ..] = &mut self.sorted_set[..] {
            let mut rest = rest.to_vec();
            rest.extend([2, 3, 5].into_iter().filter_map(|x: i32| x.checked_mul(*head)));
            rest.sort_unstable();
            rest.dedup();
            h = Some(*head);
            self.sorted_set = rest;
        }
        h
    }
}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        UglyNumbers::nth(n as usize - 1)
    }
}