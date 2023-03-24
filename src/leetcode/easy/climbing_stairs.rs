pub struct Solution;

impl Solution {
    // フィボナッチ数列と同じアプローチ
    // n=45 のときに Time Limit Exceeded してしまう‥
    pub fn climb_stairs_time_limit_exceeded(n: i32) -> i32 {
        // 2段: 1 + 1, 2
        if n <= 2 {
            return n;
        }
        Self::climb_stairs_time_limit_exceeded(n - 1)
            + Self::climb_stairs_time_limit_exceeded(n - 2)
    }

    // O(n) にした。これもフィボナッチ数列の求め方と同じ。
    pub fn climb_stairs(n: i32) -> i32 {
        let mut s1 = 0;
        let mut s2 = 1;
        let mut temp = 0;

        for _ in 0..n {
            // (s1, s2) = (s2, s1+s2) は leetcode の環境だとコンパイル出来ない
            temp = s1 + s2;
            s1 = s2;
            s2 = temp;
        }
        temp
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        struct TestCase {
            n: i32,
            output: i32,
        }
        let test_cases = [
            TestCase { n: 2, output: 2 },
            TestCase { n: 3, output: 3 },
            TestCase { n: 4, output: 5 },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::climb_stairs(tc.n));
        }
    }
}
