#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }

        let mut itr: usize = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[itr] = nums[i];
                itr += 1;
            }
        }

        for i in itr..nums.len() {
            nums[i] = 0;
        }

        nums
    }
}
