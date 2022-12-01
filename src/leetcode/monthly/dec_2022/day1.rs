#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn halves_are_alike(mut s: String) -> bool {
        s = s.to_ascii_lowercase();
        let (left, right) = s.split_at(s.len() / 2);

        let lambda = |ch: &char| Solution::is_vowel(*ch);

        let left_count = left.chars().filter(lambda).count();
        let right_count = right.chars().filter(lambda).count();

        left_count == right_count
    }

    fn is_vowel(ch: char) -> bool {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        assert_eq!(Solution::halves_are_alike(String::from("book")), true);
    }
}
