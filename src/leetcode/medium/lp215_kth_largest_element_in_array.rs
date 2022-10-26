#![allow(dead_code)]

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for &num in nums.iter() {
            heap.push(Reverse(num));
            if heap.len() as i32 == k + 1 {
                heap.pop();
            }
        }
        
        let result = heap.pop();

        result.unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;

        assert_eq!(Solution::find_kth_largest(nums, k), 5);
    }
}
