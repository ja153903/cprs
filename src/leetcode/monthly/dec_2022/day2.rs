#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        // two strings are considered close if they have the same characters
        // they can also be considered close if there exists a bijection
        let mut w1_counter: Vec<i32> = vec![0; 26];
        let mut w2_counter: Vec<i32> = vec![0; 26];
        let mut frequencies: HashMap<i32, i32> = HashMap::new();

        for (w1_ch, w2_ch) in word1.chars().zip(word2.chars()) {
            let w1_code = (w1_ch as u32) - 97;
            w1_counter[w1_code as usize] += 1;

            let w2_code = (w2_ch as u32) - 97;
            w2_counter[w2_code as usize] += 1;
        }

        for (&w1_count, &w2_count) in w1_counter.iter().zip(w2_counter.iter()) {
            // any time we have a character that exists in one but not the other
            // we return false;
            if w1_count > 0 && w2_count == 0 || w2_count > 0 && w1_count == 0 {
                return false;
            }

            *frequencies.entry(w1_count).or_insert(0) += 1;
            *frequencies.entry(w2_count).or_insert(0) -= 1;
        }

        // if the frequencies don't cancel out, then we know that they are not close
        for &value in frequencies.values() {
            if value != 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let word1 = String::from("abc");
        let word2 = String::from("bca");

        assert!(Solution::close_strings(word1, word2));
    }

    #[test]
    pub fn should_pass_sample_case2() {
        let word1 = String::from("cabbba");
        let word2 = String::from("abbccc");

        assert!(Solution::close_strings(word1, word2));
    }

    #[test]
    pub fn should_pass_sample_case3() {
        let word1 = String::from("cabbba");
        let word2 = String::from("aabbss");

        assert!(!Solution::close_strings(word1, word2));
    }

    #[test]
    pub fn should_pass_sample_case4() {
        let word1 = String::from("uau");
        let word2 = String::from("ssx");

        assert!(!Solution::close_strings(word1, word2));
    }
}
