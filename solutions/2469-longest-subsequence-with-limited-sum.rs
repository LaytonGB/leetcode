impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        for i in 1..nums.len() {
            nums[i] = nums[i-1] + nums[i];
        }
        println!("{:?}", nums);
        let mut res = Vec::new();
        for n in queries.iter() {
            let r = nums.binary_search(n);
            res.push(if r.is_ok() {r.unwrap() + 1} else {r.unwrap_err()} as i32);
        }
        res
    }
}