#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn mirror_reflection(_p: i32, _q: i32) -> i32 {
        // TODO: Don't understand this problem
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test] #[ignore]
    pub fn should_pass_sample_case() {
        assert_eq!(Solution::mirror_reflection(2, 1), 2);
        assert_eq!(Solution::mirror_reflection(3, 1), 1);
    }
}
