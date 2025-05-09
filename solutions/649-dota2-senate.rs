impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut s = senate.chars().collect::<Vec<char>>();
        let (mut r_block, mut d_block) = (0, 0);
        loop {
            if r_block > s.len() { return "Dire".to_owned(); }
            if d_block > s.len() { return "Radiant".to_owned(); }
            let mut next = vec![];
            for c in s {
                match c {
                    'R' if r_block > 0 => { r_block -= 1; },
                    'R' => { next.push(c); d_block += 1; },
                    'D' if d_block > 0 => { d_block -= 1; },
                    'D' => { next.push(c); r_block += 1; },
                    _ => unreachable!(),
                }
            }
            s = next;
        }
    }
}