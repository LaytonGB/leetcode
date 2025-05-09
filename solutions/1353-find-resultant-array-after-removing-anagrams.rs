impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut last_count: [usize; 26] = [0; 26];
        words.iter().filter(|w| {
            let mut counter = [0; 26];
            w.bytes().for_each(|c| {
                counter[(c - b'a') as usize] += 1;
            });
            let res = counter != last_count;
            last_count = counter;
            res
        }).map(|s| s.to_string()).collect()
    }
}