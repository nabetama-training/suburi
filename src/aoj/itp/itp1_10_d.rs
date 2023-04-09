pub struct Solution;

impl Solution {
    pub fn solve(n: usize, a: Vec<i32>, b: Vec<i32>) -> Vec<f32> {
        // // ミンコフスキー距離を求めるため、(項)1/p 乗よりも前の計算をしておく
        let temp = (0..n)
            .map(|i| (a[i] - b[i]).abs() as f32)
            .collect::<Vec<_>>();

        let result = vec![
            temp.iter().sum(),
            temp.iter().map(|x| x.powi(2)).sum::<f32>().sqrt(),
            temp.iter().map(|x| x.powi(3)).sum::<f32>().cbrt(),
            // f32 には max がないので、cloned してから fold する
            temp.iter().cloned().fold(f32::MIN, f32::max),
        ];

        result
    }
}

trait FloatExt {
    fn max(self, other: Self) -> Self;
}

// もし f32 に max を実装するなら…
impl FloatExt for f32 {
    fn max(self, other: f32) -> f32 {
        self.partial_cmp(&other).map_or_else(
            || other,
            |order| match order {
                std::cmp::Ordering::Greater => self,
                _ => other,
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            n: usize,
            a: Vec<i32>,
            b: Vec<i32>,
            output: Vec<f32>,
        }
        let test_cases = [TestCase {
            n: 3,
            a: vec![1, 2, 3],
            b: vec![2, 0, 4],
            output: vec![4.0, 2.4494898, 2.1544347, 2.0],
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.n, tc.a, tc.b));
        }
    }
}
