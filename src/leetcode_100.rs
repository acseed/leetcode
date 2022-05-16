// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::{RefCell};

struct Solution;

impl Solution {
    fn compare(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                let p_node_ref = p_node.borrow();
                let q_node_ref = q_node.borrow();
                p_node_ref.val == q_node_ref.val
                    && Self::compare(&p_node_ref.left, &q_node_ref.left)
                    && Self::compare(&p_node_ref.right, &q_node_ref.right)
            }
            _ => false
        }
    }

    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::compare(&p, &q)
    }
}

#[test]
fn test_solution() {
    let mut result = Solution::is_same_tree(None, None);
    assert_eq!(result, true);

    let p = make_tree();
    let q = make_tree();
    result = Solution::is_same_tree(p, q);
    assert_eq!(result, true);

    let p = make_tree();
    let q = make_another_tree();
    result = Solution::is_same_tree(p, q);
    assert_eq!(result, false);

}

fn make_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let mut p_root = TreeNode::new(1);
    let p_left = TreeNode::new(2);
    let p_right = TreeNode::new(3);
    let p_left_ptr = Some(Rc::new(RefCell::new(p_left)));
    let p_right_ptr = Some(Rc::new(RefCell::new(p_right)));
    p_root.left = p_left_ptr;
    p_root.right = p_right_ptr;
    Some(Rc::new(RefCell::new(p_root)))
}

fn make_another_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let mut p_root = TreeNode::new(1);
    let p_left = TreeNode::new(2);
    let p_left_ptr = Some(Rc::new(RefCell::new(p_left)));
    p_root.left = p_left_ptr;
    Some(Rc::new(RefCell::new(p_root)))
}