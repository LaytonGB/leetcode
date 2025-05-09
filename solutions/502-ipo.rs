use std::collections::*;
impl Solution {
    pub fn find_maximized_capital(
        mut k: i32,
        mut capital: i32,
        profits: Vec<i32>,
        capitals: Vec<i32>
    ) -> i32 {
        let n = profits.len();
        let mut pq_cap: Vec<(i32, i32)> = (0..n).into_iter()
            .map(|i| (capitals[i], profits[i]))
            .collect();
        pq_cap.sort_unstable();
        // println!("{:?}", pq_cap);
        let mut ptr = 0;
        let mut pq_pro = BinaryHeap::new();
        while k > 0 {
            while ptr < n && pq_cap[ptr].0 <= capital {
                pq_pro.push(pq_cap[ptr].1);
                ptr += 1;
            }
            // println!("{:?}", pq_pro);

            if let Some(p) = pq_pro.pop() {
                capital += p;
                k -= 1;
            } else {
                break;
            }
        }
        capital
    }
}