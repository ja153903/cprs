#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        nums.sort();

        for i in 0..nums.len() - 2 {
            if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
                let mut j = i + 1;
                let mut k = nums.len() - 1;

                while j < k {
                    let current = nums[i] + nums[j] + nums[k];

                    if current == 0 {
                        result.push(vec![nums[i], nums[j], nums[k]]);
                        while j < k && nums[j] == nums[j + 1] {
                            j += 1;
                        }

                        while j < k && nums[k] == nums[k - 1] {
                            k -= 1;
                        }

                        j += 1;
                        k -= 1;
                    } else if current < 0 {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
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
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        assert_eq!(Solution::three_sum(nums), expected);
    }
}
