const ADJ: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::dfs(&mut board, &word, i, j, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], i: usize, j: usize, k: usize) -> bool {
        if k >= word.len() {
            return true;
        }
        if i >= board.len() || j >= board[0].len() || board[i][j] != word[k] {
            return false;
        }

        let temp = board[i][j];
        board[i][j] = '_';

        for (r, c) in ADJ {
            let (i, j) = (i + r, j + c);

            if Self::dfs(board, word, i, j, k + 1) {
                return true;
            }
        }

        board[i][j] = temp;

        false
    }
}