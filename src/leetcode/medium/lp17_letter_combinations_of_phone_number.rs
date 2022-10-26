#![allow(dead_code)]

use std::collections::{VecDeque, HashMap};

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let mut digit_to_chars: HashMap<char, String> = HashMap::new();
        digit_to_chars.insert('2', String::from("abc"));
        digit_to_chars.insert('3', String::from("def"));
        digit_to_chars.insert('4', String::from("ghi"));
        digit_to_chars.insert('5', String::from("jkl"));
        digit_to_chars.insert('6', String::from("mno"));
        digit_to_chars.insert('7', String::from("pqrs"));
        digit_to_chars.insert('8', String::from("tuv"));
        digit_to_chars.insert('9', String::from("wxyz"));

        let mut queue: VecDeque<String> = VecDeque::new();

        queue.push_back(String::from(""));

        for digit in digits.chars() {
            let chars = digit_to_chars.get(&digit).unwrap();
            let size = queue.len();

            for _ in 0..size {
                let front = queue.pop_front().unwrap();
                for ch in chars.chars() {
                    queue.push_back(format!("{}{}", front, ch));
                }
            }
        }

        Vec::from(queue)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let digits = String::from("23");

        let result = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(Solution::letter_combinations(digits), result);
    }
}
