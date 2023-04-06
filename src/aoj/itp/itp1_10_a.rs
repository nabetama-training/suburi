pub struct Solution;

impl Solution {
    pub fn solve(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
        // 三平方の定理
        let dx = x1 - x2;
        let dy = y1 - y2;
        ((dx.pow(2) + dy.pow(2)) as f64).sqrt()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            x1: i32,
            y1: i32,
            x2: i32,
            y2: i32,
            output: f64,
        }
        let test_cases = [TestCase {
            x1: 0,
            y1: 0,
            x2: 1,
            y2: 1,
            #[allow(clippy::approx_constant)]
            output: 1.4142135623730951,
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.x1, tc.y1, tc.x2, tc.y2));
        }
    }
}
