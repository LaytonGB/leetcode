#[derive(Debug, Clone, PartialEq, Eq)]
struct CharCounts(pub [usize; 26]);

impl CharCounts {
    #[inline]
    pub fn new() -> Self {
        Self([0; 26])
    }

    #[inline]
    pub fn add(&mut self, byte: u8) {
        self.0[(byte - b'a') as usize] += 1;
    }

    pub fn add_string(&mut self, string: String) {
        for b in string.into_bytes() {
            self.add(b);
        }
    }
}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let n = words.len();
        let mut char_counts = CharCounts::new();
        for word in words {
            char_counts.add_string(word);
        }

        char_counts.0.into_iter().all(|count| count % n == 0)
    }
}