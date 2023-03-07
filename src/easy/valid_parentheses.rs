pub fn is_valid(s: String) -> bool {
    // 空文字は false
    if s.is_empty() {
        return false;
    }

    let mut stack: Vec<char> = Vec::new(); // 判定用のテンポラリスタック

    for c in s.chars() {
        // 先頭が開きカッコだったらスタックに入れる
        if matches!(c, '(' | '[' | '{') {
            stack.push(c);
        } else if stack.is_empty() {
            // 先頭が閉じ括弧なら false
            return false;
        } else {
            // 後でpop()するのでスタックに入れてある括弧は常に左括弧
            let left = stack.last().unwrap();
            // ペアであることを確認する
            if c == ')' && *left == '(' || c == ']' && *left == '[' || c == '}' && *left == '{' {
                stack.pop(); // ペアを末尾から削除
            } else {
                return false;
            }
        }
    }

    // カッコが余ってないか
    if !stack.is_empty() {
        return false;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        #[derive(Debug)]
        struct TestCase {
            input: String,
            output: bool,
        }

        let test_cases = [
            TestCase {
                input: "".to_string(),
                output: false,
            },
            TestCase {
                input: "()".to_string(),
                output: true,
            },
            TestCase {
                input: "()[]{}".to_string(),
                output: true,
            },
            TestCase {
                input: "(]".to_string(),
                output: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(is_valid(tc.input), tc.output);
        }
    }
}
