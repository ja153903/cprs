#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen: HashMap<char, usize> = HashMap::new();
        let mut start: usize = 0;
        let mut longest_substr = 0;

        for (end, ch) in s.char_indices() {
            // when we see a character, we should check to see if we've seen the character before
            // if we've seen the character before then we should update the start of our window
            // however whether we update that window to the next value after the last seen
            // character or keep it the same should be something we take into consideration
            // if we've seen a character before but it moves our start pointer way back, then we
            // should not update the start pointer at all.
            if last_seen.contains_key(&ch) {
                start = start.max(last_seen.get(&ch).unwrap() + 1);
            }

            longest_substr = longest_substr.max((end - start + 1) as i32);
            last_seen.insert(ch, end);
        }

        longest_substr
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let s = String::from("abcabcbb");

        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let s = String::from("bbbbb");

        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    pub fn should_pass_the_last_sample_case() {
        let s = String::from("pwwkew");

        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }
}
