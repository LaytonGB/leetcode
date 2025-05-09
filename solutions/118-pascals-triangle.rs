impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        res.push(vec![1]);
        if num_rows == 1 { return res; }
        res.push(vec![1, 1]);
        if num_rows == 2 { return res; }
        for _ in 2..num_rows as usize {
            let mut vec = Vec::new();
            vec.push(1);
            let last = res.last().unwrap();
            for j in 1..last.len() {
                vec.push(last[j-1] + last[j]);
            }
            vec.push(1);
            res.push(vec);
        }
        res
    }
}