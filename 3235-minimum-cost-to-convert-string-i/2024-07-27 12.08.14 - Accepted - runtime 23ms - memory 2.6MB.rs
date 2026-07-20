impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let map = Self::get_map(original, changed, cost);
        Self::get_conversion_cost(&map, source, target)
    }

    fn get_map(original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> [[Option<i32>; 26]; 26] {
        let mut unoptimized_map = (0..26)
            .fold([[None::<i32>; 26]; 26], |mut m, a| {
                m[a][a] = Some(0);
                m
            });
        
        original.into_iter()
            .zip(changed.into_iter())
            .zip(cost.into_iter())
            .for_each(|((a, b), c)| {
                let (a, b) = ((a as u8 - b'a') as usize, (b as u8 - b'a') as usize);
                if !unoptimized_map[a][b].is_some_and(|x| x <= c) {
                    unoptimized_map[a][b].replace(c);
                }
            });

        Self::optimize_map(unoptimized_map)
    }

    fn optimize_map(mut map: [[Option<i32>; 26]; 26]) -> [[Option<i32>; 26]; 26] {
        // Floyd-Warshall algorithm
        // for all connections where b->a->c
        // if cheaper than b->c, use in place of b->c
        for a in 0..26 {
            for b in 0..26 {
                if map[b][a].is_none() {
                    continue;
                }

                for c in 0..26 {
                    if map[a][c].is_none() {
                        continue;
                    }

                    let y = map[b][a].unwrap() + map[a][c].unwrap();
                    if !map[b][c].is_some_and(|x| x <= y) {
                        map[b][c].replace(y);
                    }
                }
            }
        }

        map
    }

    fn get_conversion_cost(map: &[[Option<i32>; 26]; 26], source: String, target: String) -> i64 {
        let mut res = 0_i64;
        for (a, b) in source.bytes().zip(target.bytes()) {
            let (a, b) = ((a - b'a') as usize, (b - b'a') as usize);
            let Some(c) = map[a][b] else {
                return -1;
            };
            res += c as i64;
        }

        res
    }
}