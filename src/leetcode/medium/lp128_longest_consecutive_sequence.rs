#![allow(dead_code)]

use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let unique: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut result = 0;

        for &num in nums.iter() {
            if !unique.contains(&(num - 1)) {
                let mut len = 0;
                let mut current = num;

                while unique.contains(&current) {
                    current += 1;
                    len += 1;
                }

                result = result.max(len);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let expected = 4;

        assert_eq!(Solution::longest_consecutive(nums), expected);
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let expected = 9;

        assert_eq!(Solution::longest_consecutive(nums), expected);
    }
}
