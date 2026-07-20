#[derive(Debug, Clone, Default, PartialEq)]
struct Cash {
    pub tens: usize,
    pub fives: usize,
}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut cash = Cash::default();
        for bill in bills {
            match bill {
                5 => cash.fives += 1,
                10 => if cash.fives < 1 {
                    return false;
                } else {
                    cash.fives -= 1;
                    cash.tens += 1;
                }
                20 => match (cash.tens, cash.fives) {
                    (0, ..=2) => return false,
                    (0, 3..) => {
                        cash.fives -= 3;
                    }
                    (1.., 0) => return false,
                    (1.., 1..) => {
                        cash.tens -= 1;
                        cash.fives -= 1;
                    }
                }
                _ => unreachable!("Customers can only hand over 5s, 10s, and 20s"),
            }
        }
        true
    }
}