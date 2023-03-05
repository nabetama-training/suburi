#[cfg(test)]
mod test {
    #[test]
    fn test_roman_to_int() {
        struct TestCase {
            input: String,
            output: i32,
        }

        let test_cases = [
            TestCase {
                input: "III".to_string(),
                output: 3,
            },
            TestCase {
                input: "LVIII".to_string(),
                output: 58,
            },
            TestCase {
                input: "MCMXCIV".to_string(),
                output: 1994,
            },
        ];

        for tc in test_cases {
            assert_eq!(roman_to_int(tc.input), tc.output);
        }
    }
}
