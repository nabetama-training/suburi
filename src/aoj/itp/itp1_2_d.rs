pub struct Solution;

impl Solution {
    pub fn solve(w: i32, h: i32, x: i32, y: i32, r: i32) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            w: i32,
            h: i32,
            x: i32,
            y: i32,
            r: i32,
            output: bool,
        }
        let test_cases = [
            TestCase {
                w: 5,
                h: 4,
                x: 2,
                y: 2,
                r: 1,
                output: true,
            },
            TestCase {
                w: 5,
                h: 4,
                x: 2,
                y: 4,
                r: 1,
                output: false,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.w, tc.h, tc.x, tc.y, tc.r));
        }
    }
}
