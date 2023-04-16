use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut count = 0;
    let mut a = a;

    loop {
        // すべてが2の倍数であるか
        if a.iter().all(|&x| x % 2 == 0) {
            // すべての要素を2で割って、新しい配列を作る
            a = a.iter().map(|&x| x / 2).collect();
            // 2で割った回数をカウント
            count += 1;
        } else {
            break;
        }
    }
    println!("{}", count);
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
                input: "3\n8 12 40".to_string(),
                output: "2\n".to_string(),
            },
            TestCase {
                input: "4\n5 6 8 10".to_string(),
                output: "0\n".to_string(),
            },
            TestCase {
                input: "6\n382253568 723152896 37802240 379425024 404894720 471526144".to_string(),
                output: "8\n".to_string(),
            },
        ];

        for tc in test_cases {
            assert_cmd::Command::cargo_bin("abc081_b")
                .unwrap()
                .write_stdin(tc.input)
                .assert()
                .success()
                .stdout(tc.output);
        }
    }
}
