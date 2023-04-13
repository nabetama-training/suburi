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
        let input = "101";
        let output = "2\n";
        assert_cmd::Command::cargo_bin("abc081_a")
            .unwrap()
            .write_stdin(input)
            .assert()
            .success()
            .stdout(output);
    }
}
