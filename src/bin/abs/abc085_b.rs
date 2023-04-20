use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [i32; n]
    }

    d.sort();

    let mut count = 1;

    for i in 1..n {
        if d[i] != d[i - 1] {
            count += 1;
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
                input: r#"4
10
8
8
6
"#,
                output: r#"3
"#,
            },
            TestCase {
                input: r#"7
50
30
50
100
50
80
30
"#,
                output: r#"4
"#,
            },
        ];

        for tc in test_cases.iter() {
            assert_cmd::Command::cargo_bin("abc085_b")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
