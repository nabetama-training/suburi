pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut i = digits.len() - 1;

    loop {
        if digits[i] < 9 {
            digits[i] += 1;
            break;
        }

        // digits[i] が　9以上だったら繰り上げ
        digits[i] = 0;

        // i == 0 は先頭の要素
        // 5行目の処理で break してないということは、先頭まで順に繰り上がって来ている。
        // なので先頭に1を挿入する
        if i == 0 {
            digits.insert(0, 1);
            break;
        }
        i -= 1;
    }
    digits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus_one() {
        struct TestCase {
            input: Vec<i32>,
            output: Vec<i32>,
        }
        let test_cases = [
            TestCase {
                input: vec![1, 2, 3],
                output: vec![1, 2, 4],
            },
            TestCase {
                input: vec![4, 3, 2, 1],
                output: vec![4, 3, 2, 2],
            },
            TestCase {
                input: vec![9],
                output: vec![1, 0],
            },
            TestCase {
                input: vec![9, 9],
                output: vec![1, 0, 0],
            },
            TestCase {
                input: vec![1, 9, 9],
                output: vec![2, 0, 0],
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, plus_one(tc.input));
        }
    }
}
