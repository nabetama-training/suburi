pub struct Solution;

impl Solution {
    pub fn solve(s: String) -> String {
        let mut result = String::new();
        for c in s.chars() {
            match c.is_lowercase() {
                true => result.push(c.to_uppercase().next().unwrap()),
                false => result.push(c.to_lowercase().next().unwrap()),
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
            s: String,
            output: String,
        }
        let test_cases = [TestCase {
            s: "fAIR, LATER, OCCASIONALLY CLOUDY.".to_string(),
            output: "Fair, later, occasionally cloudy.".to_string(),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.s));
        }
    }
}
