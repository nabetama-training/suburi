pub struct Solution;

impl Solution {
    pub fn solve(h: i32, w: i32) -> String {
        if h == 0 && w == 0 {
            return String::new();
        }

        let mut rectangle = String::new();

        for i in 0..h {
            if i == 0 || i == h - 1 {
                println!("{}", "#".to_string().repeat(w as usize));
                rectangle += &"#".to_string().repeat(w as usize);
                rectangle += "\n";
            } else {
                println!("{}", "#".to_string().to_frame(w as usize));
                rectangle += &"#".to_string().to_frame(w as usize);
                rectangle += "\n";
            }
        }
        println!();
        rectangle += "\n";
        rectangle
    }
}

trait Framer {
    fn to_frame(&self, n: usize) -> String;
}

impl Framer for String {
    fn to_frame(&self, n: usize) -> String {
        let mut result = String::new();
        result.push_str(self);

        for _ in 0..n - 2 {
            result.push('.');
        }
        result.push_str(self);
        result
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
                output: "####\n#..#\n####\n\n".to_string(),
            },
            TestCase {
                h: 5,
                w: 6,
                output: "######\n#....#\n#....#\n#....#\n######\n\n".to_string(),
            },
            TestCase {
                h: 3,
                w: 3,
                output: "###\n#.#\n###\n\n".to_string(),
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

    #[test]
    fn test_to_frame() {
        assert_eq!("#".to_string().to_frame(5), "#...#");
    }
}
