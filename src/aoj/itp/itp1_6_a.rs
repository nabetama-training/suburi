pub struct Solution;

impl Solution {
    pub fn solve(n: i32, numbers: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for i in (0..n).rev() {
            result.push(numbers[i as usize]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            n: i32,
            numbers: Vec<i32>,
            output: Vec<i32>,
        }
        let test_cases = [
            TestCase {
                n: 5,
                numbers: vec![1, 2, 3, 4, 5],
                output: vec![5, 4, 3, 2, 1],
            },
            TestCase {
                n: 8,
                numbers: vec![3, 3, 4, 4, 5, 8, 7, 9],
                output: vec![9, 7, 8, 5, 4, 4, 3, 3],
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.n, tc.numbers));
        }
    }
}
