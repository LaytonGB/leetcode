use std::cmp::Ordering;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut start, mut total, mut tank) = (0,0,0);
        for (i,(g,c)) in gas.into_iter().zip(cost.into_iter()).enumerate() {
            tank = tank + g - c;
            if tank < 0 {
                start = i + 1;
                total += tank;
                tank = 0;
            }
        }
        if total + tank < 0 {-1} else {start as i32}
    }
}