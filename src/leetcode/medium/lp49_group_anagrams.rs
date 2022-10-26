#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut by_anagram: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.iter() {
            let mut chars = s.chars().collect::<Vec<char>>();
            chars.sort();

            let sorted_str = chars.iter().collect::<String>();

            by_anagram
                .entry(sorted_str)
                .or_insert(Vec::new())
                .push(s.clone());
        }

        by_anagram
            .values()
            .map(|val| val.clone())
            .collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut result = Solution::group_anagrams(strs);
        result.sort();
        let expected = vec![
            vec![String::from("bat")],
            vec![
                String::from("eat"),
                String::from("tea"),
                String::from("ate"),
            ],
            vec![String::from("tan"), String::from("nat")],
        ];

        assert_eq!(result, expected);
    }
}
