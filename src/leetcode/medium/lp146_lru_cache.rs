#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

struct LRUCache {
    /// lru holds the keys in the order they're inserted
    /// That being said, the Least Recently Used value should be the last item in the deque
    lru: VecDeque<i32>,
    cache: HashMap<i32, i32>,
    capacity: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            lru: VecDeque::new(),
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.cache.contains_key(&key) {
            -1
        } else {
            if let Some(&value) = self.cache.get(&key) {
                self.update_value(key);
                value
            } else {
                -1
            }
        }
    }

    fn update_value(&mut self, value: i32) {
        self.lru.retain(|&num| num != value);
        self.lru.push_front(value);
    }

    fn put(&mut self, key: i32, value: i32) {
        // in the case that the key already exists
        if self.cache.contains_key(&key) {
            self.cache.insert(key, value);
            self.update_value(key);
            return;
        }

        if self.lru.len() as i32 == self.capacity {
            let lru = self.lru.pop_back().unwrap();
            self.cache.remove(&lru);
        }

        self.cache.insert(key, value);
        self.lru.push_front(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_simple_case() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);

        assert_eq!(lru_cache.get(1), 1);

        lru_cache.put(3, 3);

        assert_eq!(lru_cache.get(2), -1);

        lru_cache.put(4, 4);

        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }

    #[test]
    pub fn should_pass_another_case() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 0);
        lru_cache.put(2, 2);

        assert_eq!(lru_cache.get(1), 0);

        lru_cache.put(3, 3);

        assert_eq!(lru_cache.get(2), -1);

        lru_cache.put(4, 4);

        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }
}
