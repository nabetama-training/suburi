pub struct Solution;

impl Solution {
    pub fn solve(n: i32) -> String {
        let mut result = " ".to_string();

        for i in 1..=n {
            if i % 3 == 0 {
                result.push_str(&i.to_string());
                result.push(' ');
                continue;
            }
            if i % 10 == 3 {
                result.push_str(&i.to_string());
                result.push(' ');
            }
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
            n: i32,
            output: String,
        }
        let test_cases = [TestCase {
            n: 30,
            output: " 3 6 9 12 13 15 18 21 23 24 27 30 ".to_string(),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.n));
        }
    }
}
