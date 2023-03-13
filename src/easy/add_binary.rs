// 2進数から10進数に変換する
// → overflow するのでNG
pub fn s_to_i(s: &str) -> i128 {
    let mut num: i128 = 0;

    for (i, c) in s.chars().rev().enumerate() {
        if c == '1' {
            num += 2_i128.pow(i as u32);
        }
    }

    num
}

pub fn add_binary(a: String, b: String) -> String {
    let a_i128: i128 = s_to_i(&a);
    let b_i128 = s_to_i(&b);

    let sum = a_i128 + b_i128;

    format!("{sum:b}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_s_to_i() {
        assert_eq!(3, s_to_i("11"));
    }

    #[test]
    fn test_add_binary() {
        struct TestCase {
            input1: String,
            input2: String,
            output: String,
        }
        let test_cases = [
            TestCase {
                input1: "11".to_string(),
                input2: "1".to_string(),
                output: "100".to_string(),
            },
            TestCase {
                input1: "1010".to_string(),
                input2: "1011".to_string(),
                output: "10101".to_string(),
            },
            // TestCase {
            //     input1: "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(),
            //     input2: "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string(),
            //     output: "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000".to_string(),
            // },
            // TestCase {
            //     input1: "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(),
            //     input2: "1".to_string(),
            //     output: "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(),
            // },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, add_binary(tc.input1, tc.input2));
        }
    }
}
