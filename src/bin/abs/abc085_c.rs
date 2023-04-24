use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    // N 枚のお札の合計金額が Y 円となることがありえない場合は、-1 -1 -1 と出力するのでデフォルト値を -1 にしておく
    let mut man_yen = -1;
    let mut gosen_yen = -1;
    let mut sen_yen = -1;

    // 10000 円札の枚数を 0 から N 枚まで試す
    for i in 0..=n {
        // 5000 円札の枚数を 0 から N - 10000 円札の枚数まで試す
        for j in 0..=n - i {
            // 1000 円札の枚数は N - 10000 円札の枚数 - 5000 円札の枚数になる
            let k = n - i - j;
            // 3種類のお札の合計金額が Y 円になる場合は、それぞれの枚数を出力する
            if 10000 * i + 5000 * j + 1000 * k == y {
                man_yen = i;
                gosen_yen = j;
                sen_yen = k;
            }
        }
    }
    // 3種類のお札の枚数を出力する
    println!("{} {} {}", man_yen, gosen_yen, sen_yen)
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
                input: r#"9 45000
"#,
                output: r#"4 0 5
"#,
            },
            TestCase {
                input: r#"20 196000
"#,
                output: r#"-1 -1 -1
"#,
            },
            TestCase {
                input: r#"1000 1234000
"#,
                output: r#"26 0 974
"#,
            },
            TestCase {
                input: r#"2000 20000000
"#,
                output: r#"2000 0 0
"#,
            },
        ];

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
