use std::collections::*;
use std::rc::Rc;
use std::cell::RefCell;

struct WordDictionary {
    paths: [Option<Rc<RefCell<WordDictionary>>>; 26],
    is_word: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        WordDictionary {
            paths: [(); 26].map(|_| None),
            is_word: false,
        }
    }
    
    // word contains a-z only
    fn add_word(&mut self, word: String) {
        let mut bytes = word.bytes();
        let b = bytes.next().unwrap();
        if self.paths[(b - b'a') as usize].is_none() {
            self.paths[(b - b'a') as usize] = Some(Rc::new(RefCell::new(Self::new())));
        }
        let mut curr = self.paths[(b - b'a') as usize].clone().unwrap();
        for b in bytes {
            if curr.borrow().paths[(b - b'a') as usize].is_none() {
                curr.borrow_mut().paths[(b - b'a') as usize] = Some(Rc::new(RefCell::new(Self::new())));
            }
            let temp = curr.clone();
            let temp = temp.borrow();
            curr = temp.paths[(b - b'a') as usize].clone().unwrap();
        }
        curr.borrow_mut().is_word = true;
    }
    
    // word contains a-z or .
    fn search(&self, word: String) -> bool {
        self.slice_search(&word.as_bytes())
    }

    fn slice_search(&self, word: &[u8]) -> bool {
        if word.is_empty() {
            self.is_word
        } else if word[0] == b'.' {
            self.paths.iter().any(|p| {
                if let Some(p) = p {
                    let p = p.borrow();
                    p.slice_search(&word[1..])
                } else {
                    false
                }
            })
        } else if let Some(p) = &self.paths[(word[0] - b'a') as usize] {
            p.borrow().slice_search(&word[1..])
        } else {
            false
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */