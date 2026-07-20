impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        Self::h(&arr, start as usize, &mut vec![false; arr.len()])
    }

    fn h(arr: &[i32], start: usize, vis: &mut [bool]) -> bool {
        if arr[start] == 0 {
            return true;
        }

        if vis[start] {
            return false;
        }
        vis[start] = true;

        if start - (arr[start] as usize) < arr.len() {
            if Self::h(arr, start - (arr[start] as usize), vis) {
                return true;
            }
        }

        if start + (arr[start] as usize) < arr.len() {
            if Self::h(arr, start + (arr[start] as usize), vis) {
                return true;
            }
        }

        false
    }
}