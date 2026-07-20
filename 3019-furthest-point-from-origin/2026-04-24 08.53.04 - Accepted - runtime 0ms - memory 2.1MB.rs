impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        moves.chars()
            .fold([0, 0], |[l, r]: [i32; 2], m| {
                match m {
                    'L' => [l + 1, r - 1],
                    'R' => [l - 1, r + 1],
                    '_' => [l + 1, r + 1],
                    _ => unreachable!(),
                }
            })
            .into_iter()
            .max()
            .unwrap()
    }
}