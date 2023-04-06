pub struct Solution;

impl Solution {
    pub fn solve(s: String, order: String, args: String) -> String {
        let mut s = s;
        let args = args.split_whitespace().collect::<Vec<&str>>();

        match order.as_str() {
            "replace" => {
                let a = args[0].parse::<usize>().unwrap() - 1;
                let b = args[1].parse::<usize>().unwrap() - 1;
                let p = args[2].to_string();
                s.replace_range(a..=b, &p);
            }
            "reverse" => {
                let a = args[0].parse::<usize>().unwrap();
                let b = args[1].parse::<usize>().unwrap() - 1;
                if a == 0 {
                    let mut p = s[0..=b].to_string();
                    p = p.chars().rev().collect();
                    s.replace_range(0..=b, &p);
                    return s;
                }
                let mut p = s[a..=b].to_string();
                p = p.chars().rev().collect();
                s.replace_range(a..=b, &p);
            }
            "print" => {
                let a = args[0].parse::<usize>().unwrap() - 1;
                let b = args[1].parse::<usize>().unwrap() - 1;
                return s[a..=b].to_string();
            }
            _ => unreachable!("invalid order"),
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
            order: String,
            args: String,
            output: String,
        }
        let test_cases = [
            TestCase {
                s: "abcde".to_string(),
                order: "replace".to_string(),
                args: "1 3 xyz".to_string(),
                output: "xyzde".to_string(),
            },
            TestCase {
                s: "abcde".to_string(),
                order: "reverse".to_string(),
                args: "0 2".to_string(),
                output: "bacde".to_string(),
            },
            TestCase {
                s: "abcde".to_string(),
                order: "print".to_string(),
                args: "1 4".to_string(),
                output: "abcd".to_string(),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.s, tc.order, tc.args));
        }
    }
}
