#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row_start = 0;
        let mut row_end = matrix.len() - 1;
        let mut col_start = 0;
        let mut col_end = matrix[0].len() - 1;
        let mut result: Vec<i32> = Vec::new();

        while row_start <= row_end && col_start <= col_end {
            // go from left to right on the first row
            for col in col_start..=col_end {
                result.push(matrix[row_start][col]);
            }

            row_start += 1;

            for row in row_start..=row_end {
                result.push(matrix[row][col_end]);
            }

            let (cur_col_end, is_overflow) = col_end.overflowing_sub(1);
            if is_overflow {
                break;
            }

            col_end = cur_col_end;

            if row_start <= row_end {
                for col in (col_start..=col_end).rev() {
                    result.push(matrix[row_end][col]);
                }

            }

            let (cur_row_end, is_overflow) = row_end.overflowing_sub(1);
            if is_overflow {
                break;
            }

            row_end = cur_row_end;

            if col_start <= col_end {
                for row in (row_start..=row_end).rev() {
                    result.push(matrix[row][col_start]);
                }

            }

            col_start += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];

        assert_eq!(Solution::spiral_order(matrix), expected);
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let matrix = vec![vec![1], vec![2]];
        let expected = vec![1, 2];

        assert_eq!(Solution::spiral_order(matrix), expected);
    }
}
