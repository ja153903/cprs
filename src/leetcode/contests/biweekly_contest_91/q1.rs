#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        let mut averages: HashSet<String> = HashSet::new();
        // input size is small, so lets sort this
        nums.sort();

        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let sum = nums[left] + nums[right];
            let div = sum / 2;
            let modulo = sum % 2;

            let as_str = format!("{},{}", div, modulo);
            averages.insert(as_str);

            left += 1;
            right -= 1;
        }

        averages.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let nums = vec![4, 1, 4, 0, 3, 5];
        let expected = 2;

        assert_eq!(Solution::distinct_averages(nums), expected);
    }
}
