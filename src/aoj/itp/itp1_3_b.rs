pub struct Solution;

impl Solution {
    // counter was implemented using closure
    pub fn solve(mut counter: i32) -> impl FnMut(i32) -> String {
        move |x| {
            if x == 0 {
                return "".to_string();
            }
            counter += 1;
            format!("Case{counter}: {x}")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            input: i32,
            output: String,
        }

        let test_cases = vec![
            TestCase {
                input: 3,
                output: "Case1: 3".to_string(),
            },
            TestCase {
                input: 5,
                output: "Case2: 5".to_string(),
            },
            TestCase {
                input: 4,
                output: "Case3: 4".to_string(),
            },
            TestCase {
                input: 10,
                output: "Case4: 10".to_string(),
            },
            TestCase {
                input: 0,
                output: "".to_string(),
            },
        ];

        let mut f = Solution::solve(0);
        for test_case in test_cases {
            assert_eq!(test_case.output, f(test_case.input))
        }
    }
}
