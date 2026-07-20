use std::cmp::Ordering as O;

#[derive(Debug, Clone)]
struct TrieNode {
    len: usize,
    pos: usize,
    conn: [Option<Box<TrieNode>>; 26],
}

impl<'a> TrieNode {
    fn new() -> Self {
        Self {
            len: usize::MAX,
            pos: usize::MAX,
            conn: std::array::from_fn(|_| None),
        }
    }

    fn update(&mut self, len: usize, pos: usize) {
        match self.len.cmp(&len) {
            O::Less => {}
            O::Equal => {}
            O::Greater => {
                self.len = len;
                self.pos = pos;
            }
        };
    }

    fn merge(&mut self, len: usize, pos: usize, mut bytes: impl Iterator<Item = u8>) {
        self.update(len, pos);

        let Some(b) = bytes.next() else {
            return;
        };
        
        if let Some(mut next) = self.conn[(b - b'a') as usize].as_mut() {
            next.merge(len, pos, bytes);
        } else {
            let mut next = Self::new();
            next.merge(len, pos, bytes);
            self.conn[(b - b'a') as usize] = Some(Box::new(next));
        }
    }

    fn find_longest_match_index(&self, mut bytes: impl Iterator<Item = u8>) -> usize {
        if let Some(b) = bytes.next()
        && let Some(next) = &self.conn[(b - b'a') as usize] {
            let maybe_res = next.find_longest_match_index(bytes);
            if maybe_res == usize::MAX {
                self.pos
            } else {
                maybe_res
            }
        } else {
            self.pos
        }
    }
}

#[derive(Debug, Clone)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn find_longest_match_index(&self, other: String) -> usize {
        self.root.find_longest_match_index(other.bytes().rev())
    }
}

impl From<Vec<String>> for Trie {
    fn from(value: Vec<String>) -> Self {
        let mut root = TrieNode::new();

        for (pos, s) in value.into_iter().enumerate() {
            root.merge(s.len(), pos, s.bytes().rev());
        }

        Self {
            root
        }
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let trie = Trie::from(words_container);
        words_query.into_iter()
            .map(|s| trie.find_longest_match_index(s) as i32)
            .collect()
    }
}