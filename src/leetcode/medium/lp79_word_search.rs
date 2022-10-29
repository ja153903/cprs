#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Some(board[i][j]) == word.chars().nth(0) {
                    if Solution::dfs(&mut board, i as i32, j as i32, &word, 0) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, row: i32, col: i32, word_ref: &str, current: usize) -> bool {
        if row < 0 || col < 0 || row >= board.len() as i32 || col >= board[0].len() as i32 {
            return false;
        }

        if Some(board[row as usize][col as usize]) != word_ref.chars().nth(current) {
            return false;
        }

        if word_ref.len() - 1 == current {
            return true;
        }

        let temp = board[row as usize][col as usize];
        board[row as usize][col as usize] = '\0';

        let result = Solution::dfs(board, row + 1, col, word_ref, current + 1)
            || Solution::dfs(board, row - 1, col, word_ref, current + 1)
            || Solution::dfs(board, row, col + 1, word_ref, current + 1)
            || Solution::dfs(board, row, col - 1, word_ref, current + 1);

        board[row as usize][col as usize] = temp;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let board: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCCED");

        let result = Solution::exist(board, word);

        assert!(result);
    }
}
