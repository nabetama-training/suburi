use proconio::input;

fn main() {
    input! {
        a: i32, // 500円
        b: i32, // 100円
        c: i32, // 50円
        x: i32, // 合計金額
    }

    let mut count = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
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
                input: r#"2
2
2
100
"#,
                output: r#"2
"#,
            },
            TestCase {
                input: r#"5
1
0
150
"#,
                output: r#"0
"#,
            },
        ];

        for tc in test_cases.iter() {
            assert_cmd::Command::cargo_bin("abc087_b")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
