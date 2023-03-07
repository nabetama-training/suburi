#[cfg(test)]
mod test {

    #[test]
    fn test_is_valid() {
        #[derive(Debug)]
        struct TestCase {
            input: String,
            output: bool,
        }

        let test_cases = [
            TestCase {
                input: "()".to_string(),
                output: true,
            },
            TestCase {
                input: "()[]{}".to_string(),
                output: true,
            },
            TestCase {
                input: "(]".to_string(),
                output: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(is_valid(tc.input), tc.output);
        }
    }
}
