pub struct Solution;

impl Solution {
    /// 三角形の面積、周の長さ、高さを求める
    /// # Arguments
    /// * `a` - 三角形の底辺の長さ
    /// * `b` - 三角形の斜辺の長さ
    /// * `c` - 三角形の底辺の角度
    /// # Returns
    /// (面積, 周の長さ, 高さ)
    pub fn solve(a: i32, b: i32, c: i32) -> (f32, f32, f32) {
        // 三角比を用いて高さを求める
        let h = b as f32 * (c as f32).to_radians().sin();
        // 三角形の面積
        let s = (a as f32) * h / 2.0;
        // 余弦定理をを用いて周の長さを求める
        let rad = (c as f32).to_radians();
        let l =
            ((a.pow(2) as f32 * b.pow(2) as f32) - 2.0 * a as f32 * b as f32 * rad.cos()).sqrt();
        (s, l, h)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            a: i32,
            b: i32,
            c: i32,
            output: (f32, f32, f32),
        }
        let test_cases = [TestCase {
            a: 4,
            b: 3,
            c: 90,
            output: (6.0, 12.0, 3.0),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.a, tc.b, tc.c));
        }
    }
}
