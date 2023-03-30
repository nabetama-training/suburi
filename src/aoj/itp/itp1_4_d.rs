pub struct Solution;

impl Solution {
    pub fn solve(n: i32, mut numbers: Vec<i32>) -> (i32, i32, i32) {
        numbers.sort();
        (
            numbers[0],
            numbers[n as usize - 1],
            numbers.into_iter().sum(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            n: i32,
            numbers: Vec<i32>,
            output: (i32, i32, i32),
        }
        let test_cases = [
            TestCase {
                n: 5,
                numbers: vec![10, 1, 5, 4, 17],
                output: (1, 17, 37),
            },
            TestCase {
                n: 10,
                numbers: vec![10, 1, 5, 4, 17, 8, 24, 9, 128, 3],
                output: (1, 128, 209),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.n, tc.numbers));
        }
    }
}
