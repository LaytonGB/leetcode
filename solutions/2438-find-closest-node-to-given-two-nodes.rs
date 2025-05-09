impl Solution {
    pub fn closest_meeting_node(e: Vec<i32>, mut n1: i32, mut n2: i32) -> i32 {
        let n = e.len();
        let (mut d1, mut d2) = (vec![-1_i32; n], vec![-1_i32; n]);
        let mut d = 0_usize;
        while n1 != -1 && d1[n1 as usize] == -1 {
            d1[n1 as usize] = d as i32;
            d += 1;
            n1 = e[n1 as usize];
        }
        d = 0;
        while n2 != -1 && d2[n2 as usize] == -1 {
            d2[n2 as usize] = d as i32;
            d += 1;
            n2 = e[n2 as usize];
        }
        let (mut min_dist, mut res) = (usize::MAX, -1_i32);
        for i in 0..n {
            let max = d1[i].max(d2[i]);
            if d1[i].min(d2[i]) != -1 && min_dist > max as usize {
                min_dist = max as usize;
                res = i as i32;
            }
        }
        res
    }
}