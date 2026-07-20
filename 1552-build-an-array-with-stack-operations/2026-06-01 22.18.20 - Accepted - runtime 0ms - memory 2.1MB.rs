const MAX_OPS: usize = 2000;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let m = target.len();
        let mut it = (1..=n);
        let mut ptr: usize = 0;
        let mut res = Vec::with_capacity(MAX_OPS);
        while ptr < m {
            let x = it.next().unwrap();
            println!("{}", x);
            res.push("Push".to_string());
            if x >= target[ptr] {
                ptr += 1;
            } else {
                res.push("Pop".to_string());
            }
        }
        res
    }
}