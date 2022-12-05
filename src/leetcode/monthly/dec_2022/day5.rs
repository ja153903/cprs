#![allow(dead_code)]

use crate::leetcode::data_structures::ListNode;

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head.clone();

        while fast.is_some() && fast.as_ref()?.next.is_some() {
            fast = fast.unwrap().next.unwrap().next;
            slow = slow.unwrap().next;
        }

        slow
    }
}
