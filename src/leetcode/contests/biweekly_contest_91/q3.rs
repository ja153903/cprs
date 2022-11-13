#![allow(dead_code)]

struct Solution;

// TODO: Solve this problem later
impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
        let bob = 3;
        let amount = vec![-2, 4, 2, -4, 6];

        assert_eq!(Solution::most_profitable_path(edges, bob, amount), 6);
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let edges = vec![vec![0, 1]];
        let bob = 1;
        let amount = vec![-7280, 2350];

        assert_eq!(Solution::most_profitable_path(edges, bob, amount), -7280);
    }
}
