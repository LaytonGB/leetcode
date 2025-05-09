use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let n = n as usize;
        let m = meetings.len();
        meetings.sort_unstable_by_key(|x| x[2]);

        meetings.into_iter()
            .fold((Vec::<Vec<(i32, i32)>>::new(), HashSet::new()), |(mut groups, mut seen_times), meet| {
                let (x, y, t) = (meet[0], meet[1], meet[2]);

                if !seen_times.contains(&t) {
                    seen_times.insert(t);
                    groups.push(Vec::new());
                }
                groups.last_mut().unwrap().push((x, y));

                (groups, seen_times)
            })
            .0
            .into_iter()
            .fold(HashSet::from([0, first_person]), |mut res, g| {
                let mut graph = HashMap::new();
                let mut know_secret = HashSet::new();

                for (x, y) in g {
                    graph.entry(x)
                        .and_modify(|v: &mut Vec<i32>| v.push(y))
                        .or_insert(vec![y]);
                    graph.entry(y)
                        .and_modify(|v: &mut Vec<i32>| v.push(x))
                        .or_insert(vec![x]);

                    if res.contains(&x) {
                        know_secret.insert(y);
                    }
                    if res.contains(&y) {
                        know_secret.insert(x);
                    }
                }

                let mut q = VecDeque::from_iter(know_secret.drain());
                while let Some(curr) = q.pop_front() {
                    res.insert(curr);
                    if let Some(connected) = graph.get(&curr) {
                        for c in connected.iter() {
                            if !res.contains(c) {
                                res.insert(*c);
                                q.push_back(*c);
                            }
                        }
                    }
                }

                res
            })
            .into_iter()
            .collect()
    }
}