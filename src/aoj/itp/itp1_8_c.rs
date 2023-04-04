use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn solve(input: String) -> Vec<(char, i32)> {
        let mut result = HashMap::new();
        let alphabets = "abcdefghijklmnopqrstuvwxyz";

        for c in input.chars() {
            if alphabets.contains(c.to_lowercase().next().unwrap()) {
                result
                    .entry(c.to_lowercase().next().unwrap())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }

        for c in alphabets.chars() {
            result.entry(c).or_insert(0);
        }

        let mut result_vec = result.into_iter().collect::<Vec<(char, i32)>>();
        result_vec.sort_by(|a, b| a.0.cmp(&b.0));
        result_vec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            input: String,
            output: Vec<(char, i32)>,
        }
        let test_cases = [TestCase {
            input: "This is a pen.".to_string(),
            output: vec![
                ('a', 1),
                ('b', 0),
                ('c', 0),
                ('d', 0),
                ('e', 1),
                ('f', 0),
                ('g', 0),
                ('h', 1),
                ('i', 2),
                ('j', 0),
                ('k', 0),
                ('l', 0),
                ('m', 0),
                ('n', 1),
                ('o', 0),
                ('p', 1),
                ('q', 0),
                ('r', 0),
                ('s', 2),
                ('t', 1),
                ('u', 0),
                ('v', 0),
                ('w', 0),
                ('x', 0),
                ('y', 0),
                ('z', 0),
            ],
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.input));
        }
    }
}
