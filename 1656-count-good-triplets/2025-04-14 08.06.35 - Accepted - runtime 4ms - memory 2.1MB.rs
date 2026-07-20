impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n = arr.len();
        let mut res = 0;
        for i in 0..n - 2 {
            let ai = arr[i];
            for j in i + 1..n - 1 {
                let aj = arr[j];
                if ai.abs_diff(aj) as i32 > a {
                    continue;
                }
                
                for k in j + 1..n {
                    let ak = arr[k];
                    if aj.abs_diff(ak) as i32 > b
                    || ai.abs_diff(ak) as i32 > c {
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