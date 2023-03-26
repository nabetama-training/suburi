pub struct Solution;

impl Solution {
    pub fn solve(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut numbers = vec![a, b, c];

        // bubble sort
        for i in 0..numbers.len() {
            for j in 0..numbers.len() - i - 1 {
                if numbers[j + 1] < numbers[j] {
                    numbers.swap(j, j + 1)
                }
            }
        }

        numbers
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
            output: Vec<i32>,
        }
        let test_cases = [TestCase {
            a: 3,
            b: 8,
            c: 1,
            output: vec![1, 3, 8],
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.a, tc.b, tc.c));
        }
    }
}
