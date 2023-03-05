#[cfg(test)]
mod test {
    #[test]
    fn test_is_palindrome() {
        struct TestCase {
            input: i32,
            expect: bool,
        }

        // Palindrome
        // An integer is a palindrome when it reads the same forward and backward.
        // For example, 121 is a palindrome while 123 is not.
        let test_cases = [
            TestCase {
                input: 121,
                expect: true,
            },
            TestCase {
                input: -121,
                expect: false,
            },
            TestCase {
                input: 10,
                expect: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(is_palindrome(tc.input), tc.expect);
        }
    }
}
