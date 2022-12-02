#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore = "no implementation"]
    #[test]
    pub fn should_pass_sample_case() {
        let word1 = String::from("abc");
        let word2 = String::from("bca");

        assert!(Solution::close_strings(word1, word2));
    }
}
