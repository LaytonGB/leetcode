impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        Self::dfs(&mut res, &mut String::new(), &s, 0, 0, &word_dict, word_dict.clone());
        res
    }

    fn dfs(res: &mut Vec<String>, temp: &mut String, s: &str, last_word_end: usize, pos: usize, all_words: &[String], words: Vec<String>) {
        if s.len() == 0 {
            return;
        }
        
        let (c, rest) = s.split_at(1);
        let next_words = words.iter()
            .filter(|w| w.len() >= pos + 1 && &w[pos..=pos] == c)
            .collect::<Vec<_>>();
        if next_words.is_empty() {
            return;
        }
        
        temp.push_str(c);

        if let Some(matched_word) = next_words.iter().find(|&w| w.as_str() == &temp[last_word_end..=last_word_end + pos]) {
            if s.len() == 1 {
                res.push(temp.clone());
            } else {
                temp.push(' ');
                Self::dfs(res, temp, rest, last_word_end + pos + 2, 0, all_words, all_words.to_vec());
                temp.pop();
            }
        }

        Self::dfs(res, temp, rest, last_word_end, pos + 1, all_words, words);

        temp.pop();
    }
}