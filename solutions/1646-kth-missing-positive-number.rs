impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        if arr[0] > k {
            return k;
        }
        let mut l = 0;
        let mut i = 0;
        while k > 0 {
            l += 1;
            if i < arr.len() && l == arr[i] {
                i += 1;
            } else {
                k -= 1;
            }
        }
        l
    }
}