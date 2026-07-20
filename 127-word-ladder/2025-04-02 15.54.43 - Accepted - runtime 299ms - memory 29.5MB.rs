use std::collections::{HashMap, HashSet, LinkedList};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let words: HashSet<String> = HashSet::from_iter(word_list.into_iter());

        let mut q = LinkedList::from([begin_word.clone()]);
        let mut visited = HashSet::from([begin_word]);
        let mut depth = 0;
        while q.len() > 0 {
            let n = q.len();
            depth += 1;

            for _ in 0..n {
                let w = q.pop_front().unwrap();
                
                if *w == end_word {
                    return depth;
                }

                for i in 0..w.len() {
                    let mut new_word = String::from(&w);
                    for c in b'a'..=b'z' {
                        new_word.replace_range(i..=i, &(c as char).to_string());
                        if visited.insert(new_word.clone()) && words.contains(&new_word) {
                            q.push_back(new_word.clone());
                        }
                    }
                }
            }
        }

        0
    }
}