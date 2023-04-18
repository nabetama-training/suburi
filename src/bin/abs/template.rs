use proconio::input;

fn main() {
    input! {
        // n: usize,
        // a: [i32; n]
        // s: String,
    }

    todo!();
}

mod test {
    #[test]
    fn test_main() {
        struct TestCase {
            input: &'static str,
            output: &'static str,
        }

        let test_cases = [TestCase {
            input: r#""#,
            output: r#""#,
        }];

        for tc in test_cases.iter() {
            assert_cmd::Command::cargo_bin("<BIN_NAME>")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
