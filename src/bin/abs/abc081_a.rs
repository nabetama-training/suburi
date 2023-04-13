use proconio::input;

fn main() {
    input! {
        s: String

    }
    println!("{}", s.chars().filter(|&c| c == '1').count());
}

mod test {
    #[test]
    fn test_main() {
        struct TestCase {
            input: String,
            output: String,
        }

        let test_cases = [
            TestCase {
                input: "101".to_string(),
                output: "2\n".to_string(),
            },
            TestCase {
                input: "000".to_string(),
                output: "0\n".to_string(),
            },
        ];

        for tc in test_cases {
            assert_cmd::Command::cargo_bin("abc081_a")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
