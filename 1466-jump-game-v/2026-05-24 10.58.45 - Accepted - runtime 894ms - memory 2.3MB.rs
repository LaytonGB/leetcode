impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;

        let n = arr.len();
        let mut res = 1;
        for i in 0..n {
            res = res.max(Self::h(n, &arr, &mut vec![None; n], i, d));
        }
        res
    }

    fn h(n: usize, arr: &[i32], memo: &mut [Option<i32>], start: usize, d: usize) -> i32 {
        if let Some(m) = memo[start] {
            return m;
        }
        
        let mut res = 0;

        // dfs left
        for i in 1..=d {
            if start - i >= n || arr[start-i] >= arr[start] {
                break;
            }

            res = res.max(Self::h(n, arr, memo, start - i, d));
        }

        // dfs right
        for i in 1..=d {
            if start + i >= n || arr[start+i] >= arr[start] {
                break;
            }
            
            if let Some(m) = memo[start+i] {
                res = res.max(m);
                continue;
            }

            res = res.max(Self::h(n, arr, memo, start + i, d));
        }

        memo[start] = Some(res + 1);

        res + 1
    }
}