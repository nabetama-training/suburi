pub struct Solution;

impl Solution {
    pub fn solve(w: String, t: String) -> i32 {
        t.split_whitespace().filter(|word| word == &w).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            w: String,
            t: String,
            output: i32,
        }
        let test_cases = [TestCase {
            w: "computer".to_string(),
            t: "Nurtures computer scientists and highly skilled computer engineers
who will create and exploit knowledge for the new era
Provides an outstanding computer environment
END_OF_TEXT"
                .to_string(),
            output: 3,
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.w, tc.t));
        }
    }
}
