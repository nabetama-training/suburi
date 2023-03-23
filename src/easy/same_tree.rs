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
    // 2つの二分木pとqの根が与えられたとき、それらが同じかどうかをチェックする関数
    // 2つの二分木の構造は同一であり、ノードが同じ値を持つ場合、同一とみなす。
    // 深さ優先探索で実装する
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    fn to_tree(numbers: Option<Vec<Option<i32>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(values) = numbers {
            let mut nodes = vec![];
            for value in values {
                // Vec<Option<Rc<RefCell<TreeNode>>>>型のベクタを作る
                nodes.push(value.map(|v| Rc::new(RefCell::new(TreeNode::new(v)))));
            }

            // 完全二分木を作る。親ノードだけわかれば子ノードが決まるので半分だけループする
            // 以下のノードを考える（1, 2, 3 の順にノードが並んでいる）
            //    1
            //   / \
            //  2   3
            // i=0番目のノード(1)に対して...
            // 左の子ノードは `2 * i + 1` 番目のノード → 2
            // 右の子ノードは `2 * i + 2` 番目のノード → 3
            for i in 0..nodes.len() / 2 {
                if let Some(ref node) = nodes[i] {
                    // 左の子ノード
                    node.borrow_mut().left = nodes[2 * i + 1].clone();
                    node.borrow_mut().right = nodes[2 * i + 2].clone();
                }
            }

            nodes[0].clone()
        } else {
            None
        }
    }

    #[test]
    fn test_is_same_tree() {
        struct TestCase {
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
            output: bool,
        }

        let test_cases = [
            TestCase {
                p: to_tree(Some(vec![Some(1), Some(2), Some(3)])),
                q: to_tree(Some(vec![Some(1), Some(2), Some(3)])),
                output: true,
            },
            TestCase {
                p: to_tree(Some(vec![Some(1), Some(2), None])),
                q: to_tree(Some(vec![Some(1), None, Some(2)])),
                output: false,
            },
            TestCase {
                p: to_tree(Some(vec![Some(1), Some(2), Some(1)])),
                q: to_tree(Some(vec![Some(1), Some(1), Some(2)])),
                output: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.output, Solution::is_same_tree(tc.p, tc.q));
        }
    }
}
