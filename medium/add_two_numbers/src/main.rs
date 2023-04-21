use std::borrow::{BorrowMut};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut l1 = l1;
        let mut l2 = l2;

        // l3_node is a mutable reference to the current node in l3
        let mut l3_node = l3.borrow_mut();
        while l1.is_some() || l2.is_some() {
            let mut sum = 0;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            if let Some(node) = l3_node {
                node.val += sum;
                // if the sum is greater than 10, we need to carry the 1 to the next node
                if node.val >= 10 {
                    node.val -= 10;
                    node.next = Some(Box::new(ListNode::new(1)));
                } else {
                    if l1.is_some() || l2.is_some() {
                        node.next = Some(Box::new(ListNode::new(0)));
                    }
                }
                l3_node = node.next.borrow_mut();
            }
            sum = 0;
        }
        
        l3
    }
}
  
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        // [2,4,3]
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(
                ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(3)))
                }
            ))
        }));
        // [5,6,4]
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(
                ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode::new(4)))
                }
            ))
        }));
        // [7,0,8]
        let expect = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(
                ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(8)))
                }
            ))
        }));
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expect);
    }
}
