use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    println!("{} {}", n, y);
}

mod test {
    #[test]
    fn test_main() {
        struct TestCase {
            input: &'static str,
            output: &'static str,
        }

        let test_cases = [TestCase {
            input: r#"9 45000
"#,
            output: r#"9 45000
"#,
        }];

        for tc in test_cases.iter() {
            assert_cmd::Command::cargo_bin("abc085_c")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
