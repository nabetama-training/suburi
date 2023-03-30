pub struct Solution;

impl Solution {
    pub fn solve(a: i32, op: String, b: i32) -> i32 {
        match &*op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            a: i32,
            op: String,
            b: i32,
            output: i32,
        }
        let test_cases = [
            TestCase {
                a: 1,
                op: "+".to_string(),
                b: 2,
                output: 3,
            },
            TestCase {
                a: 56,
                op: "-".to_string(),
                b: 18,
                output: 38,
            },
            TestCase {
                a: 13,
                op: "*".to_string(),
                b: 2,
                output: 26,
            },
            TestCase {
                a: 100,
                op: "/".to_string(),
                b: 10,
                output: 10,
            },
            TestCase {
                a: 27,
                op: "+".to_string(),
                b: 81,
                output: 108,
            },
            TestCase {
                a: 0,
                op: "?".to_string(),
                b: 0,
                output: 0,
            },
        ];
        for tc in test_cases {
            assert_eq!(
                tc.output,
                Solution::solve(tc.a, tc.op.clone(), tc.b),
                "{} {} {}",
                tc.a,
                tc.op,
                tc.b,
            );
        }
    }
}
