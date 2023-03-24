use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inner(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                inner(&node.left, result);
                result.push(node.val);
                inner(&node.right, result);
            }
        }

        let mut result = Vec::new();
        inner(&root, &mut result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 二分木を作成するヘルパー関数
    fn create_tree_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_empty() {
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        assert_eq!(Solution::inorder_traversal(root), vec![]);
    }

    #[test]
    fn test_inorder_traversal() {
        let left = create_tree_node(2, create_tree_node(1, None, None), None);
        let right = create_tree_node(3, None, None);
        let root = create_tree_node(4, left, right);

        assert_eq!(Solution::inorder_traversal(root), vec![1, 2, 4, 3]);
    }

    #[test]
    fn test_single_node() {
        let root = create_tree_node(1, None, None);
        assert_eq!(Solution::inorder_traversal(root), vec![1]);
    }

    #[test]
    fn test_left() {
        let root = create_tree_node(
            3,
            create_tree_node(2, create_tree_node(1, None, None), None),
            None,
        );
        assert_eq!(Solution::inorder_traversal(root), vec![1, 2, 3]);
    }

    #[test]
    fn test_right() {
        let root = create_tree_node(
            1,
            None,
            create_tree_node(2, None, create_tree_node(3, None, None)),
        );
        assert_eq!(Solution::inorder_traversal(root), vec![1, 2, 3]);
    }
}
