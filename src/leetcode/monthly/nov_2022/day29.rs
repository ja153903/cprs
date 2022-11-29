#![allow(dead_code)]

use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    by_index: HashMap<i32, usize>,
    values: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            by_index: HashMap::new(),
            values: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.by_index.contains_key(&val) {
            false
        } else {
            self.by_index.insert(val, self.values.len());
            self.values.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.by_index.get(&val) {
            self.values.swap_remove(index);
            if let Some(&swapped_val) = self.values.get(index) {
                self.by_index.insert(swapped_val, index);
            }
            self.by_index.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        self.values[rng.gen_range(0..self.values.len())]
    }
}
