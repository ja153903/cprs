#![allow(dead_code)]

struct Solution;

impl Solution {
    /// The solution to this problem often requires using Kadane's Algorithm
    /// This is basically a trivia question at this point.
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_so_far = nums[0];
        let mut current_max = nums[0];

        for i in 1..nums.len() {
            max_so_far = nums[i].max(nums[i] + max_so_far);
            current_max = current_max.max(max_so_far);
        }

        current_max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);
    }
}
