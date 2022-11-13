#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut sum: i64 = 0;
        let mut ans: i64 = 0;

        let mut itr: usize = 0;

        while (itr as i32) < k {
            *mp.entry(nums[itr]).or_insert(0) += 1;
            sum += nums[itr] as i64;
            itr += 1;
        }

        if mp.len() == (k as usize) {
            ans = sum;
        }

        while itr < nums.len() {
            // update the values
            mp.entry(nums[itr]).and_modify(|value| *value += 1);
            mp.entry(nums[itr - (k as usize)])
                .and_modify(|value| *value -= 1);

            if let Some(&count) = mp.get(&nums[itr - (k as usize)]) {
                if count == 0 {
                    mp.remove_entry(&nums[itr - (k as usize)]);
                }
            }

            sum += nums[itr] as i64;
            sum -= nums[itr - (k as usize)] as i64;

            if mp.len() == (k as usize) {
                ans = ans.max(sum);
            }

            itr += 1;
        }

        ans
    }
}
