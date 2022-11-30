#![allow(dead_code)]

struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counter: HashMap<i32, i32> = HashMap::new();

        for &num in arr.iter() {
            counter
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut unique_counts: HashSet<i32> = HashSet::new();

        for value in counter.values() {
            if unique_counts.contains(value) {
                return false;
            }

            unique_counts.insert(*value);
        }

        true
    }
}
