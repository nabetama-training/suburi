// Linked List の型
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (None, Some(right)) => Some(right),
        (Some(left), None) => Some(left),
        (Some(left), Some(right)) => {
            // val の大小を比較し、再帰的に並び替え続ける
            if left.val <= right.val {
                Some(Box::new(ListNode {
                    next: merge_two_lists(left.next, Some(right)),
                    val: left.val,
                }))
            } else {
                Some(Box::new(ListNode {
                    next: merge_two_lists(Some(left), right.next),
                    val: right.val,
                }))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        struct TestCase {
            input1: Option<Box<ListNode>>,
            input2: Option<Box<ListNode>>,
            output: Option<Box<ListNode>>,
        }

        let test_cases = [
            TestCase {
                input1: None,
                input2: None,
                output: None,
            },
            TestCase {
                input1: Some(Box::new(ListNode::new(13))),
                input2: None,
                output: Some(Box::new(ListNode {
                    val: 13,
                    next: None,
                })),
            },
            TestCase {
                input1: None,
                input2: Some(Box::new(ListNode::new(42))),
                output: Some(Box::new(ListNode {
                    val: 42,
                    next: None,
                })),
            },
            TestCase {
                input1: Some(Box::new(ListNode::new(13))),
                input2: Some(Box::new(ListNode::new(42))),
                output: Some(Box::new(ListNode {
                    val: 13,
                    next: Some(Box::new(ListNode::new(42))),
                })),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.output, merge_two_lists(tc.input1, tc.input2))
        }
    }
}
