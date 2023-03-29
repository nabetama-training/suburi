pub struct Solution;

impl Solution {
    pub fn solve(a: i32, b: i32) -> (i32, i32, f32) {
        let fa = a as f32;
        let fb = b as f32;
        let fc = fa / fb * 100.0 / 100.0;

        (a / b, a % b, fc)
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
            output: (i32, i32, f32),
        }
        let test_cases = [TestCase {
            a: 3,
            b: 2,
            output: (1, 1, 1.5000),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.a, tc.b));
        }
    }
}
