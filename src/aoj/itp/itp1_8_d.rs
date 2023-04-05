pub struct Solution;

impl Solution {
    pub fn solve(haystack: String, needle: String) -> bool {
        haystack.repeat(2).contains(&needle)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            haystack: String,
            needle: String,
            output: bool,
        }
        let test_cases = [TestCase {
            haystack: "vanceknowledgetoad".to_string(),
            needle: "advance".to_string(),
            output: true,
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.haystack, tc.needle));
        }
    }
}
