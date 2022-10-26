#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();

        for i in 0..matrix.len() {
            for j in i..matrix[0].len() {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        Solution::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }
}
