#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // this looks like a prefix sum problem
        // so we should have a vector for the prefix sums so that we can easily compute the difference
        let mut prefix_sums: Vec<i64> = vec![0; nums.len()];
        prefix_sums[0] = nums[0] as i64;

        for i in 1..nums.len() {
            prefix_sums[i] = prefix_sums[i - 1] + nums[i] as i64;
        }

        let mut min_avg_diff = i64::MAX;
        let mut idx_of_min_avg_diff = usize::MAX;
        let n = prefix_sums.len();

        let sum = prefix_sums.last().unwrap();

        for i in 0..prefix_sums.len() {
            // the sum of the first i + 1 elements
            let left = prefix_sums[i];
            let left_size = i + 1;
            let left_avg = left / left_size as i64;

            // the sum of the rest
            let right = sum - left;
            let right_size = n - left_size;
            let right_avg = if right_size == 0 {
                0
            } else {
                right / right_size as i64
            };

            let cur_avg_diff = (left_avg - right_avg).abs();

            if cur_avg_diff < min_avg_diff {
                min_avg_diff = cur_avg_diff;
                idx_of_min_avg_diff = i;
            }
        }

        idx_of_min_avg_diff as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let nums = vec![2, 5, 3, 9, 5, 3];

        assert_eq!(Solution::minimum_average_difference(nums), 3);
    }

    #[test]
    pub fn should_pass_sample_case2() {
        let nums = vec![0];

        assert_eq!(Solution::minimum_average_difference(nums), 0);
    }
}
