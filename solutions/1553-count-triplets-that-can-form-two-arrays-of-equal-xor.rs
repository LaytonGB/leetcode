impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        
        for i in 0..arr.len() - 1 {
            let mut xor = arr[i];

            for j in i + 1..arr.len() {
                xor ^= arr[j];

                if xor == 0 {
                    res += j - i;
                }
            }
        }

        res as i32
    }
}