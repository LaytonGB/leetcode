impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        (0..height.len())
            .fold(
                (
                    (0..height.len())
                        .rev()
                        .fold((0, vec![0; height.len()]), |(mut m, mut r), i| {
                            if height[i] > m {
                                m = height[i];
                            }
                            r[i] = m;
                            (m, r)
                        })
                        .1,
                    0,
                    0,
                ),
                |(r, mut m, mut res), i| {
                    if height[i] > m {
                        m = height[i];
                    }
                    res += m.min(r[i]) - height[i];
                    (r, m, res)
                },
            )
            .2
    }
}