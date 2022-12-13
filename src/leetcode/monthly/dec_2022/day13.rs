#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_pass_sample_test() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(Solution::min_falling_path_sum(matrix), 13);
    }

    #[test]
    fn should_pass_sample_test2() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        assert_eq!(Solution::min_falling_path_sum(matrix), -59);
    }
}
