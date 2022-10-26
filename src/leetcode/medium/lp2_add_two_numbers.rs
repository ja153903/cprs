#![allow(dead_code)]

use crate::leetcode::data_structures::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut runner = &mut head;

        let mut carry: i32 = 0;

        while l1.is_some() || l2.is_some() {
            let value: i32 = match (&l1, &l2) {
                (Some(v1), Some(v2)) => v1.val + v2.val + carry,
                (None, Some(v2)) => v2.val + carry,
                (Some(v1), None) => v1.val + carry,
                (None, None) => carry,
            };

            carry = value / 10;
            runner.next = Some(Box::new(ListNode::new(value % 10)));
            runner = runner.next.as_mut().unwrap();

            l1 = if l1.is_some() { l1.unwrap().next } else { l1 };
            l2 = if l2.is_some() { l2.unwrap().next } else { l2 };
        }

        if carry > 0 {
            runner.next = Some(Box::new(ListNode::new(carry)));
        }

        head.next
    }
}
