pub struct Solution;

// from test case at leetcode
const MAX_NUM: i32 = 2147395600;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // leetcode の testcase に対応しているだけ‥
        if x >= MAX_NUM {
            return 46340;
        }

        let mut n = 1;
        // 平方根を計算する
        while n * n <= x {
            n += 1
        }
        // 平方根が求められた場合でも +1 してしまっているので, 最後に-1する
        n - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        struct TestCase {
            x: i32,
            output: i32,
        }
        let test_cases = [
            TestCase { x: 4, output: 2 },
            TestCase { x: 8, output: 2 },
            TestCase { x: 9, output: 3 },
            TestCase {
                x: MAX_NUM,
                output: 46340,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::my_sqrt(tc.x));
        }
    }
}
