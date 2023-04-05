pub struct Solution;

impl Solution {
    pub fn solve(s: String, shuffle_times: Vec<i32>) -> String {
        let mut s = s;
        for shuffle_time in shuffle_times {
            let (left, right) = s.split_at(shuffle_time as usize);
            s = format!("{right}{left}");
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            s: String,
            shuffle_times: Vec<i32>,
            output: String,
        }
        let test_cases = [
            TestCase {
                s: "aabc".to_string(),
                shuffle_times: vec![1, 2, 1],
                output: "aabc".to_string(),
            },
            TestCase {
                s: "vwxyz".to_string(),
                shuffle_times: vec![3, 4],
                output: "xyzvw".to_string(),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.s, tc.shuffle_times));
        }
    }
}
