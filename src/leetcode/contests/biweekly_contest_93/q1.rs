#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter()
            .map(|s| {
                let is_digit_only = s.chars().all(|c| c.is_ascii_digit());
                if is_digit_only {
                    s.parse::<i32>().unwrap()
                } else {
                    s.len() as i32
                }
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let strs = vec!["alic3", "bob", "3", "4", "00000"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let expected = Solution::maximum_value(strs);

        assert_eq!(expected, 5);
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let strs = vec!["1", "01", "001", "0001"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let expected = Solution::maximum_value(strs);

        assert_eq!(expected, 1);
    }
}
