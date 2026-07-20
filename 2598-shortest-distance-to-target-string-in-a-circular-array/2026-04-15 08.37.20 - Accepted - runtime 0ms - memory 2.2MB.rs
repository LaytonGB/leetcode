impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let si = start_index as usize;
        if words[si] == target {
            return 0;
        }

        let n = words.len();
        for i in 1..=n/2 {
            if words[(si+i)%n] == target
            || words[(si-i+n)%n] == target {
                return i as i32;
            }
        }

        -1
    }
}