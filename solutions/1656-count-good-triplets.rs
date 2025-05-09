impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n = arr.len();
        let mut res = 0;
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                if (arr[i] - arr[j]).abs() > a {
                    continue;
                }
                
                for k in j + 1..n {
                    if (arr[j] - arr[k]).abs() > b
                    || (arr[i] - arr[k]).abs() > c {
                        continue;
                    }

                    // println!("i:{} | j:{} | k:{}", arr[i], arr[j], arr[k]);
                    res += 1;
                }
            }
        }
        res
    }
}