impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let (p1, p2) = (&coordinates[0], &coordinates[1]);
        let get_m = |p1: &Vec<i32>, p2: &Vec<i32>| {
            if p1[0] == p2[0] { return f32::MAX; }
            (p2[1] - p1[1]) as f32 / (p2[0] - p1[0]) as f32
        };
        let get_c = |p: &Vec<i32>, m: f32| {
            if m == f32::MAX { return 0_f32; }
            p[1] as f32 - m * p[0] as f32
        };
        let m = get_m(p1, p2);
        let c = get_c(p1, m);
        if c != get_c(p2, m) {
            println!("{}!={} | m:{}", c, get_c(p2, m), m);
            return false;
        }
        for w in coordinates.windows(2).skip(1) {
            let (p1, p2) = (&w[0], &w[1]);
            println!("{}=={} | {}=={}", m, get_m(p1, p2), c, get_c(p2, m));
            if get_m(p1, p2) != m || get_c(p2, m) != c {
                return false;
            }
        }
        true
    }
}