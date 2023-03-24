pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    let mut strs = strs;

    strs.sort();
    let mut result = "".to_string();

    if strs.len() == 1 {
        return strs[0].to_string();
    }

    for (i, _) in strs[0].chars().enumerate() {
        if strs[0].chars().nth(i).unwrap() == strs[strs.len() - 1].chars().nth(i).unwrap() {
            result = result + &strs[0].chars().nth(i).unwrap().to_string();
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        struct TestCase {
            input: Vec<String>,
            output: String,
        }

        let test_cases = [
            TestCase {
                input: vec![
                    "flowers".to_string(),
                    "flow".to_string(),
                    "flight".to_string(),
                ],
                output: "fl".to_string(),
            },
            TestCase {
                input: vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
                output: "".to_string(),
            },
        ];

        for tc in test_cases {
            assert_eq!(longest_common_prefix(tc.input), tc.output);
        }
    }
}
