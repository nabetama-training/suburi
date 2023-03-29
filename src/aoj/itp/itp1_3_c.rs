pub struct Solution;

impl Solution {
    pub fn solve(x: i32, y: i32) -> Option<(i32, i32)> {
        if x == 0 && y == 0 {
            return None;
        }
        if x <= y {
            Some((x, y))
        } else {
            Some((y, x))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            x: i32,
            y: i32,
            output: Option<(i32, i32)>,
        }
        let test_cases = [
            TestCase {
                x: 3,
                y: 2,
                output: Some((2, 3)),
            },
            TestCase {
                x: 2,
                y: 2,
                output: Some((2, 2)),
            },
            TestCase {
                x: 5,
                y: 3,
                output: Some((3, 5)),
            },
            TestCase {
                x: 0,
                y: 0,
                output: None,
            },
        ];
        for tc in test_cases {
            match Solution::solve(tc.x, tc.y) {
                Some(tpl) => assert_eq!(tc.output.unwrap(), tpl),
                None => assert_eq!(tc.output, None),
            }
        }
    }
}
