use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,

    }

    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" });
}

mod test {
    #[test]
    fn test_main() {
        let input = "3 4";
        let output = "Even\n";
        assert_cmd::Command::cargo_bin("abc086_a")
            .unwrap()
            .write_stdin(input)
            .assert()
            .success()
            .stdout(output);
    }
}
