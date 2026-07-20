#[derive(Debug, PartialEq, Eq)]
struct Counts {
    chars: [usize; 26],
}

impl Counts {
    pub fn new() -> Self {
        Counts {
            chars: [0; 26],
        }
    }
}

impl FromIterator<u8> for Counts {
    fn from_iter<
        T: IntoIterator<Item = u8>
    >(iter: T) -> Self {
        iter.into_iter()
            .fold(Counts::new(), |mut res, c| {
                res.chars[(c - b'a') as usize] += 1;
                res
            })
    }
}

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let s1a = s1.bytes()
            .step_by(2)
            .collect::<Counts>();
        let s2a = s2.bytes()
            .step_by(2)
            .collect::<Counts>();
        
        let s1b = s1.bytes()
            .skip(1)
            .step_by(2)
            .collect::<Counts>();
        let s2b = s2.bytes()
            .skip(1)
            .step_by(2)
            .collect::<Counts>();
        
        s1a == s2a && s1b == s2b
    }
}