const MIN: i32 = b'a' as i32;
const MAX: i32 = b'z' as i32;
const DIFF: i32 = MAX - MIN + 1;
const LIST_LENGTH: usize = 50_001;

enum D {
    Forwards,
    Backwards,
}

impl From<i32> for D {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Forwards,
            0 => Self::Backwards,
            _ => unreachable!(),
        }
    }
}

fn rem(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut prefix = [0_i32; LIST_LENGTH];

        shifts.iter().for_each(|s| {
            debug_assert!(s.len() == 3);

            let [start, end, direction] = s[..] else {
                panic!();
            };

            let direction = D::from(direction);
            match direction {
                D::Forwards => {
                    prefix[start as usize] += 1;
                    prefix[end as usize + 1] -= 1;
                }
                D::Backwards => {
                    prefix[start as usize] -= 1;
                    prefix[end as usize + 1] += 1;
                }
            }
        });

        let mut bytes = s.as_bytes().to_vec();
        let mut modifier = 0;
        for (byte, prefix) in bytes.iter_mut().zip(prefix.into_iter()) {
            modifier += prefix;
            // println!("{:?}", (&byte, &prefix, &modifier));
            let mut b = *byte as i32 + modifier;
            while b > MAX {
                b -= DIFF;
            }
            while b < MIN {
                b += DIFF;
            }
            *byte = b as u8;
        }

        String::from_utf8(bytes).unwrap()
    }
}
