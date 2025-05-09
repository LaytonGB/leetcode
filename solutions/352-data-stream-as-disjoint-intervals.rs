use std::cmp::Ordering::{Equal, Greater, Less};

struct SummaryRanges {
    v: Vec<Vec<i32>>,
}

impl SummaryRanges {

    fn new() -> Self {
        SummaryRanges {
            v: Vec::new()
        }
    }
    
    fn add_num(&mut self, value: i32) {
        let n = self.v.len();
        if n == 0 {
            self.v.push(vec![value; 2]);
            return;
        }
        let (mut l, mut h) = (0, n);
        let mut m = 0;
        let mut range;
        while l < h {
            m = l + ((h - l) / 2);
            range = &self.v[m];
            match (value.cmp(&range[0]), value.cmp(&range[1])) {
                (Equal, _) | (_, Equal) | (Greater, Less) => return,
                (Less, _) => h = m,
                (_, Greater) => l = m + 1,
            }
        }
        if m == n {
            self.v.push(vec![value; 2]);
            return;
        }
        match (value.cmp(&self.v[m][0]), value.cmp(&self.v[m][1])) {
            (Less, _) => {
                if m == 0 {
                    if self.v[m][0] - 1 == value {
                        self.v[m][0] = value
                    } else {
                        self.v.insert(0, vec![value; 2])
                    }
                } else {
                    match (self.v[m - 1][1] + 1 == value, self.v[m][0] - 1 == value) {
                        (true, true) => {
                            self.v[m - 1][1] = self.v[m][1];
                            self.v.remove(m);
                        },
                        (true, _) => self.v[m - 1][1] = value,
                        (_, true) => self.v[m][0] = value,
                        (_, _) => self.v.insert(m, vec![value; 2]),
                    }
                }
            },
            (_, Greater) => {
                if m + 1 == n {
                    if self.v[m][1] + 1 == value {
                        self.v[m][1] = value
                    } else {
                        self.v.push(vec![value; 2])
                    }
                } else {
                    match (self.v[m][1] + 1 == value, self.v[m + 1][0] - 1 == value) {
                        (true, true) => {
                            self.v[m][1] = self.v[m + 1][1];
                            self.v.remove(m + 1);
                        },
                        (true, _) => self.v[m][1] = value,
                        (_, true) => self.v[m + 1][0] = value,
                        (_, _) => self.v.insert(m + 1, vec![value; 2]),
                    }
                }
            },
            (_, _) => unreachable!(),
        }
    }
    
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.v.clone()
    }
}