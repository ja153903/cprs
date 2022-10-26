#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result = 0;

        while left < right {
            let area = height[left].min(height[right]) * (right as i32 - left as i32);
            result = result.max(area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
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
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;

        assert_eq!(Solution::max_area(height), expected)
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let height = vec![1, 1];
        let expected = 1;

        assert_eq!(Solution::max_area(height), expected)
    }
}
