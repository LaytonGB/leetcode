#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Solution {
    fn backtrack(
        board: &mut Vec<Vec<char>>,
        node: &mut Trie,
        r: usize,
        c: usize,
        s: &mut String,
        valid: &mut Vec<String>,
    ) {
        let ch = board[r][c];
        if let Some(node) = &mut node.children[(ch as u8 - b'a') as usize] {
            s.push(ch);
            if node.is_end {
                valid.push(s.clone());
                node.is_end = false;
            }
            board[r][c] = '*';
            for adj in [0, 1, 0, !0, 0].windows(2) { // what is !0 ? TODO: play with
                let r = r.wrapping_add(adj[0]);
                let c = c.wrapping_add(adj[1]);
                // why use this instead of checking > 0 and < board lens?
                // surely checking a range is slower? TODO: study
                if (0..board.len()).contains(&r)
                    && (0..board[0].len()).contains(&c)
                    && board[r][c] != '*'
                {
                    Self::backtrack(board, node, r, c, s, valid);
                }
            }
            board[r][c] = ch;
            s.pop();
        }
    }
    
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut out = Vec::new();
        
        // trie setup
        let mut root = Trie::default();
        for w in &words {
            let mut node = &mut root;
            for b in w.bytes() {
                node = node.children[(b - b'a') as usize].get_or_insert(Default::default());
            }
            node.is_end = true;
        }

        // search start
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                Self::backtrack(
                    &mut board,
                    &mut root,
                    r,
                    c,
                    &mut String::new(),
                    &mut out,
                );
            }
        }

        out
    }
}