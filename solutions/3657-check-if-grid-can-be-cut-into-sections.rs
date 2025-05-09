use std::collections::{BTreeMap, BTreeSet};

impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut dx_start = BTreeMap::new();
        let mut dx_end = BTreeMap::new();
        let mut dx_points = BTreeSet::new();

        let mut dy_start = BTreeMap::new();
        let mut dy_end = BTreeMap::new();
        let mut dy_points = BTreeSet::new();

        for r in rectangles.iter() {
            let [sx, sy, ex, ey] = &r[..] else {unreachable!()};

            dx_start.entry(sx)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            dx_end.entry(ex)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            dx_points.insert(sx);
            dx_points.insert(ex);

            dy_start.entry(sy)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            dy_end.entry(ey)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            dy_points.insert(sy);
            dy_points.insert(ey);
        }

        let mut d = 0;
        let mut blocks_since_last_zero = 0;
        let mut zero_points = 0;
        for p in dx_points.iter() {
            let s = dx_start.get(p).unwrap_or(&0);
            let e = dx_end.get(p).unwrap_or(&0);
            
            d -= e;

            // println!("p:{} s:{} e:{} d:{} b:{}", p, s, e, d, blocks_since_last_zero);
            match (d, blocks_since_last_zero) {
                (0, 1..) => {
                    zero_points += 1;

                    if zero_points >= 3 {
                        return true;
                    }

                    blocks_since_last_zero = 0;
                }
                _ => {}
            }

            d += s;
            blocks_since_last_zero += s;
        }

        // println!("===");

        d = 0;
        blocks_since_last_zero = 0;
        zero_points = 0;
        for p in dy_points.iter() {
            let s = dy_start.get(p).unwrap_or(&0);
            let e = dy_end.get(p).unwrap_or(&0);
            
            d -= e;

            // println!("p:{} s:{} e:{} d:{} b:{}", p, s, e, d, blocks_since_last_zero);
            match (d, blocks_since_last_zero) {
                (0, 1..) => {
                    zero_points += 1;

                    if zero_points >= 3 {
                        return true;
                    }

                    blocks_since_last_zero = 0;
                }
                _ => {}
            }

            d += s;
            blocks_since_last_zero += s;
        }

        false
    }
}