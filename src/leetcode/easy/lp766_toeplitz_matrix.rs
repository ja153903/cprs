#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        // Given some pair of indices (i, j), we want to go from
        let mut queue: Vec<(usize, usize)> = vec![];

        for col in 0..matrix[0].len() {
            queue.push((0, col));
        }

        for row in 1..matrix.len() {
            queue.push((row, 0));
        }

        while !queue.is_empty() {
            if let Some((i, j)) = queue.pop() {
                let mut i = i;
                let mut j = j;
                let ref_value = matrix[i][j];

                while i < matrix.len() && j < matrix[0].len() {
                    if matrix[i][j] != ref_value {
                        return false;
                    }

                    i += 1;
                    j += 1;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![36, 59, 71, 15, 26, 82, 87],
            vec![56, 36, 59, 71, 15, 26, 82],
            vec![15, 0, 36, 59, 71, 15, 26],
        ];

        assert_eq!(Solution::is_toeplitz_matrix(matrix), false);
    }
}
