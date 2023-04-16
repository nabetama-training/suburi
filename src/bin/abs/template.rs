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
        assert_cmd::Command::cargo_bin("<BIN_NAME>")
            .unwrap()
            .write_stdin(input)
            .assert()
            .success()
            .stdout(output);
    }
}
