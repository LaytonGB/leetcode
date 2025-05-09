#[derive(Clone, Debug)]
struct DictMap {
    is_root: bool,
    map: [Option<Box<DictMap>>; 26],
}
impl DictMap {
    pub fn new() -> Self {
        Self {
            is_root: false,
            map: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
        }
    }

    pub fn add_root(&mut self, s: &str) {
        let mut dm = self;
        for b in s.bytes() {
            dm = dm.map[(b - b'a') as usize]
                .get_or_insert(Box::new(Self::new()));
        }
        dm.is_root = true;
    }

    pub fn add_all_roots(&mut self, dict: &Vec<String>) {
        for s in dict.iter() {
            self.add_root(s);
        }
    }

    pub fn find_root(&self, s: &str) -> Option<String> {
        let mut res = String::new();
        let mut dm = self;
        for b in s.bytes() {
            if dm.map[(b - b'a') as usize].is_some() {
                dm = dm.map[(b - b'a') as usize].as_ref().unwrap();
            } else {
                return None;
            }

            res.push(b as char);

            if dm.is_root {
                return Some(res);
            }
        }
        None
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut dm = DictMap::new();
        dm.add_all_roots(&dictionary);
        sentence.split_ascii_whitespace()
            .map(|s| dm.find_root(s).unwrap_or(s.to_string()))
            .reduce(|mut a, b| {
                a.push(' ');
                a.push_str(&b);
                a
            })
            .unwrap()
    }
}