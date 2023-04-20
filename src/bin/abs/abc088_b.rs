use proconio::input;

fn main() {
    input! {
        n: i32,
        cards: [i32; n],
    }

    let mut alice = 0;
    let mut bob = 0;

    let mut cards = cards;
    cards.sort_by(|a, b| b.cmp(a));

    for (i, card) in cards.iter().enumerate() {
        if i % 2 == 0 {
            alice += card;
        } else {
            bob += card;
        }
    }
    println!("{}", alice - bob);
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
3 1
"#,
                output: r#"2
"#,
            },
            TestCase {
                input: r#"3
2 7 4
"#,
                output: r#"5
"#,
            },
            TestCase {
                input: r#"4
20 18 2 18
"#,
                output: r#"18
"#,
            },
        ];

        for tc in test_cases.iter() {
            assert_cmd::Command::cargo_bin("abc088_b")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
