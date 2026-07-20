const NUMBER_WORDS: [&str; 10] = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
const TENS_WORDS: [&str; 10] = ["", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
const SIZES: [Option<&str>; 4] = [None, Some("Thousand"), Some("Million"), Some("Billion")];

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        (0..=num.ilog10())
            .map(|i| (num / 10_i32.pow(i)) % 10)
            .collect::<Vec<_>>()
            .chunks(3)
            .enumerate()
            .rev()
            .map(|(i, v)| match &v[..] {
                [a, b, c] => (i, vec![*c, *b, *a]),
                [a, b] => (i, vec![*b, *a]),
                [a] => (i, vec![*a]),
                _ => unreachable!(),
            })
            .flat_map(|(i, v)| {
                let mut v = Self::triplet_to_words(v);
                if let (Some(s), false) = (SIZES[i], v.is_empty()) {
                    v.push(s);
                }
                v
            })
            .collect::<Vec<&'static str>>()
            .join(" ")
    }

    fn triplet_to_words(v: Vec<i32>) -> Vec<&'static str> {
        let (a, b, c) = match &v[..] {
            [a, b, c] => (Some(a), Some(b), Some(c)),
            [b, c] => (None, Some(b), Some(c)),
            [c] => (None, None, Some(c)),
            _ => panic!(),
        };

        let mut s = Vec::with_capacity(3);

        if let Some(a @ 1..) = a {
            s.push(NUMBER_WORDS[*a as usize]);
            s.push("Hundred");
        }

        match (b, c) {
            (None, None) => {}
            (None, Some(c @ 1..)) | (Some(0), Some(c @ 1..)) => {
                s.push(NUMBER_WORDS[*c as usize]);
            }
            (Some(b @ 1..), None) | (Some(b @ 1..), Some(0)) => {
                s.push(TENS_WORDS[*b as usize])
            }
            (Some(1), Some(1)) => s.push("Eleven"),
            (Some(1), Some(2)) => s.push("Twelve"),
            (Some(1), Some(3)) => s.push("Thirteen"),
            (Some(1), Some(4)) => s.push("Fourteen"),
            (Some(1), Some(5)) => s.push("Fifteen"),
            (Some(1), Some(6)) => s.push("Sixteen"),
            (Some(1), Some(7)) => s.push("Seventeen"),
            (Some(1), Some(8)) => s.push("Eighteen"),
            (Some(1), Some(9)) => s.push("Nineteen"),
            (Some(b @ 1..), Some(c @ 1..)) => {
                s.push(TENS_WORDS[*b as usize]);
                s.push(NUMBER_WORDS[*c as usize]);
            }
            _ => {}
        }

        s
    }
}