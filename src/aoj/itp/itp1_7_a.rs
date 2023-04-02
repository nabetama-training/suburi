pub struct Solution;

impl Solution {
    pub fn solve(m: i32, f: i32, r: i32) -> String {
        // 終了条件
        if m == -1 && f == -1 {
            return "".to_string();
        }
        // 中間試験、期末試験のいずれかを欠席した場合成績は F
        if m == -1 || f == -1 {
            return "F".to_string();
        }
        // 中間試験と期末試験の合計点数が 80 以上ならば成績は A
        if m + f >= 80 {
            return "A".to_string();
        }
        // 中間試験と期末試験の合計点数が 65 以上ならば成績は B
        if m + f >= 65 {
            return "B".to_string();
        }
        // 中間試験と期末試験の合計点数が 50 以上ならば成績は C
        if m + f >= 50 {
            return "C".to_string();
        }
        // 中間試験と期末試験の合計点数が 30 以上ならば成績は D
        if m + f >= 30 {
            // ただし、再試験の点数が 50 以上ならば成績は C
            if r >= 50 {
                return "C".to_string();
            }
            return "D".to_string();
        }
        // その他（中間試験と期末試験の合計点数が 30 未満）ならば成績は F
        "F".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            m: i32,
            f: i32,
            r: i32,
            output: String,
        }
        let test_cases = [
            TestCase {
                m: -1,
                f: 40,
                r: -1,
                output: "F".to_string(),
            },
            TestCase {
                m: 40,
                f: 42,
                r: -1,
                output: "A".to_string(),
            },
            TestCase {
                m: 25,
                f: 40,
                r: -1,
                output: "B".to_string(),
            },
            TestCase {
                m: 24,
                f: 40,
                r: -1,
                output: "C".to_string(),
            },
            TestCase {
                m: 20,
                f: 30,
                r: -1,
                output: "C".to_string(),
            },
            TestCase {
                m: 19,
                f: 30,
                r: -1,
                output: "D".to_string(),
            },
            TestCase {
                m: 19,
                f: 30,
                r: 50,
                output: "C".to_string(),
            },
            TestCase {
                m: 0,
                f: 2,
                r: -1,
                output: "F".to_string(),
            },
            TestCase {
                m: -1,
                f: -1,
                r: -1,
                output: "".to_string(),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.m, tc.f, tc.r));
        }
    }
}
