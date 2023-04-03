pub struct Solution;

impl Solution {
    pub fn solve(n: i32, x: i32) -> i32 {
        let mut count = 0;
        // 3つ選ぶので n(O3)
        for i in 1..=n {
            for j in i + 1..=n {
                for k in j + 1..=n {
                    if i + j + k == x {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            n: i32,
            x: i32,
            output: i32,
        }
        let test_cases = [TestCase {
            n: 5,
            x: 9,
            output: 2,
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.n, tc.x));
        }
    }
}
