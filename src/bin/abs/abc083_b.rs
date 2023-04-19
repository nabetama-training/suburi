use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut sum = 0;

    for i in 1..=n {
        let mut tmp = i;
        let mut sum_tmp = 0;

        while tmp > 0 {
            sum_tmp += tmp % 10;
            tmp /= 10;
        }

        if sum_tmp >= a && sum_tmp <= b {
            sum += i;
        }
    }
    println!("{}", sum);
}

mod test {
    #[test]
    fn test_main() {
        struct TestCase {
            input: &'static str,
            output: &'static str,
        }

        let test_cases = [TestCase {
            input: r#"20
2
5"#,
            output: r#"84
"#,
        }];

        for tc in test_cases.iter() {
            assert_cmd::Command::cargo_bin("abc083_b")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
