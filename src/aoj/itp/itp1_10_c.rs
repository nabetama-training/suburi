pub struct Solution;

impl Solution {
    pub fn solve(s_num: i32, scores: Vec<i32>) -> f32 {
        let avg = scores.iter().sum::<i32>() as f32 / s_num as f32;
        // 分散
        let variance = scores.iter().map(|&x| pow2(x as f32 - avg)).sum::<f32>();
        // 標準偏差 = 分散の正の平方根
        (variance / s_num as f32).sqrt()
    }
}

pub fn pow2(x: f32) -> f32 {
    x.powf(2.0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pow2() {
        assert_eq!(pow2(2.0), 4.0);
    }

    #[test]
    fn test_solve() {
        struct TestCase {
            s_num: i32,
            scores: Vec<i32>,
            output: f32,
        }
        let test_cases = [
            TestCase {
                s_num: 5,
                scores: vec![70, 80, 100, 90, 20],
                output: 27.856777,
            },
            TestCase {
                s_num: 3,
                scores: vec![80, 80, 80],
                output: 0.0,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.s_num, tc.scores));
        }
    }
}
