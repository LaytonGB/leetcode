impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();

        let mut counts = [0; 6];
        for (t, b) in tops.iter().zip(bottoms.iter()).map(|(t, b)| (*t as usize, *b as usize)) {
            if t == b {
                counts[t - 1] += 1;
            } else {
                counts[t - 1] += 1;
                counts[b - 1] += 1;
            }
        }

        if counts.iter().all(|c| *c < n) {
            return -1;
        }

        let target = counts.iter().enumerate().max_by_key(|x| x.1).unwrap().0 as i32 + 1;
        let tops_in_place = tops.iter().filter(|t| **t == target).count();
        let bottoms_in_place = bottoms.iter().filter(|b| **b == target).count();
        let in_place = tops_in_place.max(bottoms_in_place);
        
        if n / 2 - in_place > in_place {
            (n - in_place) as i32
        } else {
            in_place as i32
        }
    }
}