#[cfg(test)]
mod test {
    #[test]
    fn test_search_insert() {
        struct TestCase {
            nums: Vec<i32>,
            target: i32,
            output: i32,
        }

        let test_cases = [
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 5,
                output: 2,
            },
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 2,
                output: 1,
            },
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 7,
                output: 4,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.output, search_insert(tc.nums, tc.target));
        }
    }
}
