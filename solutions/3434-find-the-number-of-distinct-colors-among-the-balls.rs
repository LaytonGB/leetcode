use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.iter()
            .enumerate()
            .fold(
                (
                    vec![0; queries.len()],
                    vec![0; (limit + 1) as usize],
                    HashMap::with_capacity((limit + 1) as usize)
                ),
                |(mut res, mut colors, mut counts), (index, q)| {
                    let [i, c] = &q[..] else {
                        unreachable!();
                    };

                    res[index] = *res.get(index - 1).unwrap_or(&0);

                    if *c != colors[*i as usize] {
                        let old_color_count = if colors[*i as usize] == 0 {
                            None
                        } else {
                            Some(
                                *counts.entry(colors[*i as usize])
                                    .and_modify(|count| *count -= 1)
                                    .or_insert_with(|| unreachable!())
                            )
                        };

                        if matches!(old_color_count, Some(0)) {
                            counts.remove(&colors[*i as usize]);
                            res[index] -= 1;
                        }
                        
                        counts.entry(*c)
                            .and_modify(|count| *count += 1)
                            .or_insert_with(|| {
                                res[index] += 1;
                                1
                            });

                        colors[*i as usize] = *c;
                    }

                    (res, colors, counts)
                }
            )
            .0
    }
}