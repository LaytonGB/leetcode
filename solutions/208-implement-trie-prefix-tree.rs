use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Trie {
    pub values: [Option<Rc<RefCell<Trie>>>; 26],
    pub string: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
            values: [(); 26].map(|_| None),
            string: false,
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut bytes = word.bytes();
        let temp_val = bytes.next().unwrap();
        let mut target = if self.values[(temp_val - b'a') as usize].is_some() {
            self.values[(temp_val - b'a') as usize].clone().unwrap()
        } else {
            let new_entry = Rc::new(RefCell::new(Trie::new()));
            let res = new_entry.clone();
            self.values[(temp_val - b'a') as usize] = Some(new_entry);
            res
        };
        for c in bytes {
            let mut temp = target.clone();
            let mut temp = temp.borrow_mut();
            target = if temp.values[(c - b'a') as usize].is_some() {
                temp.values[(c - b'a') as usize].clone().unwrap()
            } else {
                let new_entry = Rc::new(RefCell::new(Trie::new()));
                let res = new_entry.clone();
                temp.values[(c - b'a') as usize] = Some(new_entry);
                res
            };
        }
        target.borrow_mut().string = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut bytes = word.bytes();
        if let Some(t) = self.values[(bytes.next().unwrap() - b'a') as usize].clone() {
            let mut trie = t.clone();
            let mut next: Option<Rc<RefCell<Trie>>>;
            for c in bytes {
                if let Some(t) = trie.borrow().values[(c - b'a') as usize].clone() {
                    next = Some(t);
                } else {
                    return false;
                }
                if next.is_some() {
                    trie = next.take().unwrap();
                }
            }
            let trie = trie.borrow();
            trie.string
        } else {
            false
        }
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut bytes = prefix.bytes();
        if let Some(t) = self.values[(bytes.next().unwrap() - b'a') as usize].clone() {
            let mut trie = t.clone();
            let mut next: Option<Rc<RefCell<Trie>>>;
            for c in bytes {
                if let Some(t) = trie.borrow().values[(c - b'a') as usize].clone() {
                    next = Some(t);
                } else {
                    return false;
                }
                if next.is_some() {
                    trie = next.take().unwrap();
                }
            }
            true
        } else {
            false
        }
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */