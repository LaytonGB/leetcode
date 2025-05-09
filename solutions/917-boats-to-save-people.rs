impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();

        let mut res = 0;

        let (mut l, mut h) = (0, people.len() - 1);
        while l < h {
            if people[l] + people[h] <= limit {
                l += 1;
                h -= 1;
            } else {
                h -= 1;
            }

            res += 1;
        }

        if l == h {
            res + 1
        } else {
            res
        }
    }
}