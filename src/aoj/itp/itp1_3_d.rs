pub struct Solution;

impl Solution {
    pub fn solve(a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;
        for n in a..=b {
            if c % n == 0 {
                count += 1;
            }
        }
        count
    }
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
            output: i32,
        }
        let test_cases = [
            TestCase {
                a: 1,
                b: 10000,
                c: 10000,
                output: 25,
            },
            TestCase {
                a: 5,
                b: 14,
                c: 80,
                output: 3,
            },
            TestCase {
                a: 1,
                b: 10,
                c: 10,
                output: 4,
            },
            TestCase {
                a: 5,
                b: 25,
                c: 20,
                output: 3,
            },
            TestCase {
                a: 100,
                b: 200,
                c: 600,
                output: 4,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.a, tc.b, tc.c));
        }
    }
}
