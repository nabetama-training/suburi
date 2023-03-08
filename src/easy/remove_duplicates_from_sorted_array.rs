pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        struct TestCase {
            input: Vec<i32>,
            output: i32,
        }

        let test_cases = [
            TestCase {
                input: vec![1, 1, 2],
                output: 2,
            },
            TestCase {
                input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
                output: 5,
            },
        ];

        for mut tc in test_cases {
            assert_eq!(tc.output, remove_duplicates(&mut tc.input))
        }
    }
}
