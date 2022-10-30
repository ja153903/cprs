#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given an integer array nums of positive integers, return the average value of all even integers that are divisible by 3.
    ///  Note that the average of n elements is the sum of the n elements divided by n and rounded down to the nearest integer.
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        let mut count: i32 = 0;

        for num in nums.iter() {
            if num % 2 == 0 && num % 3 == 0 {
                sum += num;
                count += 1;
            }
        }

        if count == 0 {
            0
        } else {
            sum / count
        }
    }
}
