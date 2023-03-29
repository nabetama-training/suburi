pub struct Solution;

impl Solution {
    pub fn kiriage(n: f64) -> f64 {
        let dropped = format!("{n:.6}");
        dropped.parse::<f64>().unwrap()
    }

    pub fn solve(r: i32) -> (f64, f64) {
        use std::f64::consts::PI;
        let area = (r * r) as f64 * PI;
        let circumference = (r * 2) as f64 * PI;
        (Solution::kiriage(area), Solution::kiriage(circumference))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            r: i32,
            output: (f64, f64),
        }
        let test_cases = [
            TestCase {
                r: 2,
                output: (12.566371, 12.566371),
            },
            TestCase {
                r: 3,
                output: (28.274334, 18.849556),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.r));
        }
    }
}
