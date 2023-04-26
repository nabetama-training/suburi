use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let mut t = String::new();
    let strings = vec!["dream", "dreamer", "erase", "eraser"];

    while !s.is_empty() {
        let mut matched = false;
        for string in strings.iter() {
            if s.ends_with(string) {
                t.push_str(string);
                s = s[..s.len() - string.len()].to_string();
                matched = true;
                break;
            }
        }
        if !matched {
            break;
        }
    }
    println!("{}", if s.is_empty() { "YES" } else { "NO" });
}

mod test {
    #[test]
    fn test_main() {
        struct TestCase {
            input: &'static str,
            output: &'static str,
        }

        let test_cases = [
            TestCase {
                input: r#"erasedream
"#,
                output: r#"YES
"#,
            },
            TestCase {
                input: r#"dreameraser
"#,
                output: r#"YES
"#,
            },
            TestCase {
                input: r#"dreamerer
"#,
                output: r#"NO
"#,
            },
        ];

        for tc in test_cases.iter() {
            assert_cmd::Command::cargo_bin("arc065_a")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
