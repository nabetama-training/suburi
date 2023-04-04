pub struct Solution;

impl Solution {
    pub fn solve(num: i32) -> i32 {
        let mut result = 0;
        let mut num = num;
        while num > 0 {
            result += num % 10;
            num /= 10;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            num: i32,
            output: i32,
        }
        let test_cases = [
            TestCase {
                num: 123,
                output: 6,
            },
            TestCase {
                num: 55,
                output: 10,
            },
            TestCase {
                num: 12345,
                output: 15,
            },
            TestCase {
                num: 1000000000,
                output: 1,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.num));
        }
    }
}
