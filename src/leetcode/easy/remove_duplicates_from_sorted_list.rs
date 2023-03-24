// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(node) => {
                if let Some(next_node) = &node.next {
                    if node.val == next_node.val {
                        Self::delete_duplicates(node.next)
                    } else {
                        Some(Box::new(ListNode {
                            val: node.val,
                            next: Self::delete_duplicates(node.next),
                        }))
                    }
                } else {
                    Some(Box::new(ListNode::new(node.val)))
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn vec_to_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut current: Option<&mut Box<ListNode>> = None;

        for val in values {
            // ListNode を作る
            let new_node = Some(Box::new(ListNode::new(val)));

            match current {
                Some(node) => {
                    node.next = new_node;
                    current = node.next.as_mut();
                }
                None => {
                    head = new_node;
                    current = head.as_mut();
                }
            }
        }
        head
    }

    #[test]
    fn test_test_delete_duplicate() {
        struct TestCase {
            input: Vec<i32>,
            output: Vec<i32>,
        }

        let test_cases = [
            TestCase {
                input: vec![1, 1, 2],
                output: vec![1, 2],
            },
            TestCase {
                input: vec![1, 1, 2, 2, 3, 3],
                output: vec![1, 2, 3],
            },
            TestCase {
                input: vec![1, 1, 2, 3, 4, 5, 6, 6, 6],
                output: vec![1, 2, 3, 4, 5, 6],
            },
            TestCase {
                input: vec![1, 1, 2, 5, 7, 7, 8, 10],
                output: vec![1, 2, 5, 7, 8, 10],
            },
        ];

        for tc in test_cases {
            let input = vec_to_linked_list(tc.input);
            assert_eq!(
                Solution::delete_duplicates(input),
                vec_to_linked_list(tc.output)
            )
        }
    }
}
