#![allow(dead_code)]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        // iterate backwards on the list
        // push the results in the front when we find them
        // keep track of the current max
        let mut result: VecDeque<usize> = VecDeque::new();
        let mut current_max: i32 = *heights.last().unwrap();
        result.push_back(heights.len() - 1);

        for i in (0..heights.len() - 1).rev() {
            if current_max < heights[i] {
                result.push_front(i);
                current_max = heights[i];
            }
        }

        result.iter().map(|num| *num as i32).collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_cases() {
        let heights = vec![4, 2, 3, 1];
        let expected = vec![0, 2, 3];

        assert_eq!(Solution::find_buildings(heights), expected);
    }
}
