use std::{cell::RefCell, rc::Rc};

pub struct Solution;

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

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        struct TestCase {
            p: Option<Rc<RefCell<TreeNode>>>,
            output: bool,
        }
        let test_cases = [
            TestCase {
                p: todo!(),
                output: todo!(),
            },
            TestCase {
                p: todo!(),
                output: todo!(),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::is_same_tree(tc.p));
        }
    }
}
