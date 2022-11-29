#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        // if n is 1, then return 0
        // if n is odd in general, then we need n cuts
        // if n is even, then we need n >> 1 cuts?
        // suppose n = 6, then we have 110 => 011
        if n == 1 {
            0
        } else if n % 2 == 1 {
            n
        } else {
            n >> 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        assert_eq!(Solution::number_of_cuts(6), 3);
        assert_eq!(Solution::number_of_cuts(4), 2);
        assert_eq!(Solution::number_of_cuts(3), 3);
    }
}
