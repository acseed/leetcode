use std::borrow::Borrow;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(v1), Some(v2)) => Solution::do_add(v1, v2),
            (Some(v1), None) => Some(v1),
            (None, Some(v2)) => Some(v2),
            _ => None
        }
    }

    fn do_add(v1: Box<ListNode>, v2: Box<ListNode>) -> Option<Box<ListNode>> {
        let mut node1 = v1.clone();
        let mut node2 = v2.clone();
        let mut carry = 0;
        let mut sum = node1.val + node2.val;
        carry = sum / 10;
        let mut val = sum % 10;
        let mut head = Some(Box::new(ListNode::new(val)));
        let mut  cur = &mut head;
        let mut n1 = &node1.next;
        let mut n2 = &node2.next;
        loop {
            match (n1, n2) {
                (Some(node1), Some(node2)) => {
                    sum = node1.val + node2.val + carry;
                    carry = sum / 10;
                    val = sum % 10;
                    cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
                    cur = &mut cur.as_mut().unwrap().next;
                    n1 = &node1.next;
                    n2 = &node2.next;
                }
                (Some(node1), None) => {
                    let mut tmp = Some(node1.clone());
                    while tmp != None {
                        sum = tmp.as_mut().unwrap().val + carry;
                        carry = sum / 10;
                        val = sum % 10;
                        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
                        cur = &mut cur.as_mut().unwrap().next;
                        tmp = tmp.unwrap().next;
                    }
                    break
                }
                (None, Some(node2)) => {
                    let mut tmp = Some(node2.clone());
                    while tmp != None {
                        sum = tmp.as_mut().unwrap().val + carry;
                        carry = sum / 10;
                        val = sum % 10;
                        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
                        cur = &mut cur.as_mut().unwrap().next;
                        tmp = tmp.unwrap().next;
                    }
                    break
                }
                _ => break
            }
        }

        if carry > 0 {
            cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }

        head
    }
}

#[test]
fn test() {
    let mut l12 = Some(Box::new(ListNode::new(2)));
    let mut l14 = Some(Box::new(ListNode::new(4)));
    let mut l13 = Some(Box::new(ListNode::new(3)));
    (&mut l14).as_mut().unwrap().next = l13;
    l12.as_mut().unwrap().next = l14;
    let mut l25 = Some(Box::new(ListNode::new(5)));
    let mut l26 = Some(Box::new(ListNode::new(6)));
    let mut l24 = Some(Box::new(ListNode::new(4)));
    (&mut l26).as_mut().unwrap().next = l24;
    l25.as_mut().unwrap().next = l26;
    let result = Solution::add_two_numbers(l12, l25);
    println!("{:?}", result);
}