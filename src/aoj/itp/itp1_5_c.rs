pub struct Solution;

impl Solution {
    pub fn solve(h: i32, w: i32) -> String {
        if h == 0 && w == 0 {
            return String::new();
        }

        let mut rectangle = String::new();

        for i in 1..=h {
            rectangle.push_str(&"#".to_string().to_alternate_dot_line(w, i));
        }
        rectangle.push('\n');
        rectangle
    }
}

trait Framer {
    fn to_alternate_dot_line(&self, w: i32, line_num: i32) -> String;
}

impl Framer for String {
    fn to_alternate_dot_line(&self, w: i32, line_num: i32) -> String {
        let mut line = String::new();

        if line_num % 2 == 1 {
            for i in 0..w {
                if i % 2 == 0 {
                    line.push_str(self);
                } else {
                    line.push('.');
                }
            }
        } else {
            for i in 0..w {
                if i % 2 == 0 {
                    line.push('.');
                } else {
                    line.push_str(self);
                }
            }
        }
        line.push('\n');
        line
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_alternate_line() {
        assert_eq!("#".to_string().to_alternate_dot_line(4, 1), "#.#.\n");
        assert_eq!("#".to_string().to_alternate_dot_line(4, 2), ".#.#\n");
    }

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
                output: "#.#.\n.#.#\n#.#.\n\n".to_string(),
            },
            TestCase {
                h: 5,
                w: 6,
                output: "#.#.#.\n.#.#.#\n#.#.#.\n.#.#.#\n#.#.#.\n\n".to_string(),
            },
            TestCase {
                h: 3,
                w: 3,
                output: "#.#\n.#.\n#.#\n\n".to_string(),
            },
            TestCase {
                h: 2,
                w: 2,
                output: "#.\n.#\n\n".to_string(),
            },
            TestCase {
                h: 1,
                w: 1,
                output: "#\n\n".to_string(),
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
