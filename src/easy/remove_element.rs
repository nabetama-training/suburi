#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        struct TestCase {
            nums: Vec<i32>,
            val: i32,
            expect: i32,
        }

        let test_cases = [
            TestCase {
                nums: vec![3, 2, 2, 3],
                val: 3,
                expect: 2,
            },
            TestCase {
                nums: vec![0, 1, 2, 2, 3, 0, 4, 2],
                val: 2,
                expect: 5,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expect, remove_element(tc.nums, tc.val));
        }
    }
}
