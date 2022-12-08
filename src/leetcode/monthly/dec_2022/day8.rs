#![allow(dead_code)]

use crate::leetcode::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let r1_leaves: Vec<i32> = Solution::get_leaves(root1);
        let r2_leaves: Vec<i32> = Solution::get_leaves(root2);

        if r1_leaves.len() != r2_leaves.len() {
            false
        } else {
            for (&r1, &r2) in r1_leaves.iter().zip(r2_leaves.iter()) {
                if r1 != r2 {
                    return false;
                }
            }

            true
        }
    }

    fn get_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut by_depth = Vec::new();

        fn depth(node: Option<Rc<RefCell<TreeNode>>>, by_depth: &mut Vec<i32>) -> i32 {
            if let Some(n) = node {
                let curr = n.borrow();
                let left = depth(curr.left.clone(), by_depth);
                let right = depth(curr.right.clone(), by_depth);

                let curr_depth = 1 + left.max(right);

                if curr_depth == 0 {
                    by_depth.push(curr.val);
                }

                curr_depth
            } else {
                -1
            }
        }

        depth(root, &mut by_depth);

        by_depth
    }
}
