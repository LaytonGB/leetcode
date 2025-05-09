use std::collections::*;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let map: HashMap<usize, Vec<usize>> = prerequisites.iter()
            .fold(HashMap::new(), |mut m, p| {
                let (a, b) = (p[0] as usize, p[1] as usize);
                m.entry(a)
                    .and_modify(|v| v.push(b))
                    .or_insert(vec![b]);
                m
            });

        let mut memo = vec![false; n];
        for &a in map.keys() {
            if Self::is_cyclic(&map, &mut memo, &mut vec![false; n], a) {
                return false;
            }
        }

        true
    }

    fn is_cyclic(map: &HashMap<usize, Vec<usize>>, memo: &mut Vec<bool>, path: &mut Vec<bool>, course: usize) -> bool {
        if path[course] {
            return true;
        }
        
        path[course] = true;
        for &p in map.get(&course).unwrap_or(&vec![]) {
            if !memo[p]
            && Self::is_cyclic(map, memo, path, p) {
                return true;
            }
        }
        path[course] = false;

        memo[course] = true;
        false
    }
}