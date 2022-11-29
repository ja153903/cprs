#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut diff: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
        let mut counter: HashMap<String, i32> = HashMap::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let row_hash = format!("{}th row", i);
                    let col_hash = format!("{}th col", j);

                    *counter.entry(row_hash).or_insert(0) += 1;
                    *counter.entry(col_hash).or_insert(0) += 1;
                }
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let row_hash = format!("{}th row", i);
                let col_hash = format!("{}th col", j);

                let num_ones_in_row = counter.get(&row_hash);
                let num_ones_in_col = counter.get(&col_hash);

                let (num_ones_row, num_ones_col) = match (num_ones_in_row, num_ones_in_col) {
                    (Some(&n_row), Some(&n_col)) => (n_row, n_col),
                    (Some(&n_row), None) => (n_row, 0),
                    (None, Some(&n_col)) => (0, n_col),
                    (None, None) => (0, 0),
                };

                let num_zeros_row = cols - num_ones_row;
                let num_zeros_col = rows - num_ones_col;
                diff[i][j] = num_ones_row + num_ones_col - num_zeros_col - num_zeros_row;
            }
        }

        diff
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_test_case() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
        let expected = vec![vec![5, 5, 5], vec![5, 5, 5]];

        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }

    #[test]
    pub fn should_pass_sample_test_case2() {
        let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        let expected = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }

    #[test]
    pub fn should_pass_sample_test_case3() {
        let grid = vec![vec![0]];
        let expected = vec![vec![-2]];

        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }
}
