impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst) = (n as usize, src as usize, dst as usize);

        let mut bf = vec![None; n]; // bellman ford algorithm result storage
        bf[src] = Some(0);

        for _ in 0..=k {
            let mut temp = bf.clone();
            for f in &flights {
                let (from, to, price) = (f[0] as usize, f[1] as usize, f[2]);
                let Some(cost) = bf[from] else {
                    continue;
                };
                if temp[to].map_or(true, |old_cost| old_cost > cost + price) {
                    temp[to] = Some(cost + price);
                }
            }
            bf = temp;
        }

        bf[dst].unwrap_or(-1)
    }
}