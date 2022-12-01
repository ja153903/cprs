#![allow(dead_code)]

struct Solution;

impl Solution {
    // TODO: Solve this problem
    pub fn longest_palindrome(s: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    #[ignore = "no implementation"]
    #[test]
    pub fn should_pass_sample_case() {
        let s = String::from("babad");
        let mut expected: HashSet<String> = HashSet::new();
        expected.insert(String::from("aba"));
        expected.insert(String::from("bab"));

        let result = Solution::longest_palindrome(s);

        assert!(expected.contains(&result));
    }
}
