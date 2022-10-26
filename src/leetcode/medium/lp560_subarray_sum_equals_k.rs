#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sums: HashMap<i32, i32> = HashMap::new();
        let mut running_sum = 0;
        let mut result = 0;

        sums.insert(0, 1);

        for &num in nums.iter() {
            running_sum += num;

            if let Some(&count) = sums.get(&(running_sum - k)) {
                result += count;
            }

            sums.entry(running_sum)
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let nums = vec![1, 1, 1];
        let k = 2;

        assert_eq!(Solution::subarray_sum(nums, k), 2);
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let nums = vec![1, 2, 3];
        let k = 3;

        assert_eq!(Solution::subarray_sum(nums, k), 2);
    }
}
