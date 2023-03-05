#[allow(dead_code)]
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }
    x.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .eq(&x.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

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
