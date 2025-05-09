use std::convert::TryFrom;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if s.len() == 1 {
            return s;
        }
        let mut v: Vec<char> = s.chars().collect();
        if k == 1 {
            // build vec of considerable lists
            let mut considering: Vec<char> = Vec::from([(b'z' + 1_u8) as char]);
            let mut rotation = 0;
            for i in 0..v.len() { // for each possible rotation
                // reasons to skip:
                    // the last char == the first char (better if last is grouped with first)
                if v[0] == v[v.len()-1] {
                    v.rotate_left(1);
                    continue;
                }

                // keep by fist chars having lowest u8
                for (a,b) in v.iter().zip(considering.iter()) {
                    // after first char, keep least occurrences of char
                    let a = *a as u8;
                    let b = *b as u8;
                    if a < b {
                        considering = v.clone();
                        rotation = i;
                        break;
                    } else if a > b {
                        break;
                    }
                }
                v.rotate_left(1);
            } // at end of loop, v is in rotation zero
            v.rotate_left(rotation); // rotate to the point of the considered array
            v.iter().collect()
        } else {
            v.sort();
            v.iter().collect()
        }
    }
}