impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        std::thread::scope(|s| {
            let l = s.spawn(|| Self::get_first_diff_idx(&colors)).join().unwrap();
            let r = s.spawn(|| Self::get_last_diff_idx(&colors)).join().unwrap();
            r.max(colors.len()-1-l) as i32
        })
    }

    fn get_first_diff_idx(colors: &Vec<i32>) -> usize {
        (0..colors.len())
            .find(|&i| colors[colors.len()-1] != colors[i])
            .unwrap()
    }

    fn get_last_diff_idx(colors: &Vec<i32>) -> usize {
        (0..colors.len()).rev()
            .find(|&i| colors[0] != colors[i])
            .unwrap()
    }
}