use std::{cmp::max, collections::HashMap};

pub fn roman_to_int(s: String) -> i32 {
    let mut total = 0;

    let roman_map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let romans = s.chars().collect::<Vec<char>>();

    for (i, &c) in romans.iter().enumerate() {
        let current = roman_map[&c];

        // 右側に記号があるか?
        if i + 1 < romans.len() {
            let next = roman_map[&romans[i + 1]];

            // 現在の値が右側の記号の値以上 → 現在の記号の値を合計に加算する
            if current >= next {
                total += current;
            } else {
                // 現在の値が右側の記号の値未満 → 合計から現在の値を減算する
                total -= current;
            }
        } else {
            // 右側に記号がなければ現在の値を合計に加算
            total += current;
        }
    }
    total
}

pub fn roman_to_int2(s: String) -> i32 {
    let mut total = 0;
    let mut largest = 0;

    for c in s.chars().rev() {
        let next_c = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        largest = max(largest, next_c);

        if next_c < largest {
            total -= next_c;
        } else {
            total += next_c;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        struct TestCase {
            input: String,
            output: i32,
        }

        let test_cases = [
            TestCase {
                input: "III".to_string(),
                output: 3,
            },
            TestCase {
                input: "LVIII".to_string(),
                output: 58,
            },
            TestCase {
                input: "MCMXCIV".to_string(),
                output: 1994,
            },
        ];

        for tc in test_cases {
            assert_eq!(roman_to_int(tc.input), tc.output);
        }
    }

    #[test]
    fn test_roman_to_int2() {
        struct TestCase {
            input: String,
            output: i32,
        }

        let test_cases = [
            TestCase {
                input: "III".to_string(),
                output: 3,
            },
            TestCase {
                input: "LVIII".to_string(),
                output: 58,
            },
            TestCase {
                input: "MCMXCIV".to_string(),
                output: 1994,
            },
        ];

        for tc in test_cases {
            assert_eq!(roman_to_int2(tc.input), tc.output);
        }
    }
}
