pub struct Solution;

impl Solution {
    pub fn solve(h: i32, w: i32) -> String {
        if h == 0 && w == 0 {
            return "".to_string();
        }

        let mut rectangle = "".to_string();

        for _ in 0..h {
            for _ in 0..w {
                rectangle += "#";
                print!("#");
            }
            rectangle += "\n";
            println!();
        }
        rectangle += "\n";
        println!();
        rectangle
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            h: i32,
            w: i32,
            output: String,
        }
        let test_cases = [
            TestCase {
                h: 3,
                w: 4,
                output: "####\n####\n####\n\n".to_string(),
            },
            TestCase {
                h: 5,
                w: 6,
                output: "######\n######\n######\n######\n######\n\n".to_string(),
            },
            TestCase {
                h: 2,
                w: 2,
                output: "##\n##\n\n".to_string(),
            },
            TestCase {
                h: 0,
                w: 0,
                output: "".to_string(),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.h, tc.w));
        }
    }
}
