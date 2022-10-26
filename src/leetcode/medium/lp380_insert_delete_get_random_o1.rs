#![allow(dead_code)]

use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct RandomizedSet {
    index_by_values: HashMap<i32, usize>,
    values: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            index_by_values: HashMap::new(),
            values: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.index_by_values.contains_key(&val) {
            false
        } else {
            self.index_by_values.insert(val, self.values.len());
            self.values.push(val);

            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.index_by_values.contains_key(&val) {
            // position of the value we want to remove
            let desired = *self.index_by_values.get(&val).unwrap();

            // remove the value from the map
            self.index_by_values.remove(&val);
            // swap the value at position desired with the last element and then remove
            self.values.swap_remove(desired);

            if let Some(&value) = self.values.get(desired) {
                self.index_by_values.insert(value, desired);
            }

            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();

        self.values[rng.gen_range(0..self.values.len())]
    }
}
