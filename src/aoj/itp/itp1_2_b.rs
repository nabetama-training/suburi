pub fn solve(a: i32, b: i32, c: i32) -> String {
    if a < b && b < c {
        return "Yes".to_string();
    }
    "No".to_string()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            a: i32,
            b: i32,
            c: i32,
            expect: String,
        }

        let test_cases = [TestCase {
            a: 1,
            b: 2,
            c: 3,
            expect: "Yes".to_string(),
        }];

        for tc in test_cases {
            assert_eq!(tc.expect, solve(tc.a, tc.b, tc.c));
        }
    }
}
