#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut graph: HashMap<i32, BinaryHeap<Reverse<i32>>> = HashMap::new();

        for edge in edges.iter() {
            let u = edge[0];
            let v = edge[1];

            graph
                .entry(u)
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(vals[v as usize]));

            if let Some(heap) = graph.get_mut(&u) {
                if heap.len() > k as usize {
                    heap.pop();
                }
            }

            graph
                .entry(v)
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(vals[u as usize]));

            if let Some(heap) = graph.get_mut(&v) {
                if heap.len() > k as usize {
                    heap.pop();
                }
            }
        }

        let mut max_star_sum = i32::MIN;

        for (i, &value) in vals.iter().enumerate() {
            let mut current_star_sum = value;

            if let Some(heap) = graph.get_mut(&(i as i32)) {
                while !heap.is_empty() {
                    if let Some(v) = heap.pop() {
                        if current_star_sum + v.0 < current_star_sum {
                            continue;
                        }

                        current_star_sum += v.0;
                    }
                }
            }

            max_star_sum = max_star_sum.max(current_star_sum);
        }

        max_star_sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let vals = vec![1, 2, 3, 4, 10, -10, -20];
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![1, 3],
            vec![3, 4],
            vec![3, 5],
            vec![3, 6],
        ];
        let k = 2;

        assert_eq!(Solution::max_star_sum(vals, edges, k), 16);
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let vals = vec![1, -8, 0];
        let edges = vec![vec![0, 1], vec![1, 2]];
        let k = 2;

        assert_eq!(Solution::max_star_sum(vals, edges, k), 1);
    }
}
