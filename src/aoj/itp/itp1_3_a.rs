pub struct Solution;

impl Solution {
    pub fn solve(n: Option<i32>) -> String {
        if let Some(n_max) = n {
            let mut text = "Hello\n".repeat(n_max as usize);
            text.pop(); // 文末の改行コードを削除する
            return text;
        }
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            n: Option<i32>,
            output: String,
        }
        let test_cases = [TestCase {
            n: Some(2),
            output: "Hello\nHello".to_string(),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.n));
        }
    }
}
