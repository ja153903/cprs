#![allow(dead_code)]

use crate::leetcode::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let current = node.borrow();

                if low <= current.val && current.val <= high {
                    current.val
                        + Solution::range_sum_bst(current.left.clone(), low, high)
                        + Solution::range_sum_bst(current.right.clone(), low, high)
                } else if current.val < low {
                    Solution::range_sum_bst(current.right.clone(), low, high)
                } else {
                    Solution::range_sum_bst(current.left.clone(), low, high)
                }
            }
        }
    }
}
