#[derive(Debug, Clone, Copy)]
struct Dice {
    one: i32,
    two: i32,
    three: i32,
    four: i32,
    five: i32,
    six: i32,
}

impl Dice {
    pub fn north(&self) -> Dice {
        Dice {
            one: self.two,
            two: self.six,
            three: self.three,
            four: self.four,
            five: self.one,
            six: self.five,
        }
    }

    #[allow(dead_code)]
    pub fn south(&self) -> Dice {
        Dice {
            one: self.five,
            two: self.one,
            three: self.three,
            four: self.four,
            five: self.six,
            six: self.two,
        }
    }

    pub fn east(&self) -> Dice {
        Dice {
            one: self.four,
            two: self.two,
            three: self.one,
            four: self.six,
            five: self.five,
            six: self.three,
        }
    }
    #[allow(dead_code)]
    pub fn west(&self) -> Dice {
        Dice {
            one: self.three,
            two: self.two,
            three: self.six,
            four: self.one,
            five: self.five,
            six: self.four,
        }
    }

    pub fn right(&self) -> Dice {
        Dice {
            one: self.one,
            two: self.four,
            three: self.two,
            four: self.five,
            five: self.three,
            six: self.six,
        }
    }

    pub fn is_equal(&self, other: Dice) -> bool {
        let mut other = other;

        // itp1_11_b 同様に self.one が other.one になるように回転させる(0 <= n <= 6)
        let top_patterns = vec!["", "N", "N", "N", "E", "EE"];

        for pattern in top_patterns {
            match pattern {
                "N" => {
                    other = other.north();
                }
                "E" => {
                    other = other.east();
                }
                "EE" => {
                    other = other.east().east();
                }
                _ => {}
            }
            if self.one == other.one {
                break;
            }
        }

        // 横回転させて正面を洗い出す(0 <= n <= 4)
        let h_patterns = vec!["", "R", "RR", "RRR"];

        for h_pattern in h_patterns {
            for pattern in h_pattern.chars() {
                if pattern == 'R' {
                    other = other.right();
                }
                if self.two == other.two
                    && self.three == other.three
                    && self.four == other.four
                    && self.five == other.five
                    && self.six == other.six
                {
                    return true;
                }
            }
        }
        false
    }
}

pub struct Solution;

impl Solution {
    pub fn solve(a: Vec<i32>, b: Vec<i32>) -> bool {
        let a = Dice {
            one: a[0],
            two: a[1],
            three: a[2],
            four: a[3],
            five: a[4],
            six: a[5],
        };
        let b = Dice {
            one: b[0],
            two: b[1],
            three: b[2],
            four: b[3],
            five: b[4],
            six: b[5],
        };
        a.is_equal(b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            a: Vec<i32>,
            b: Vec<i32>,
            output: bool,
        }
        let test_cases = [
            TestCase {
                a: vec![1, 2, 3, 4, 5, 6],
                b: vec![6, 2, 4, 3, 5, 1],
                output: true,
            },
            TestCase {
                a: vec![1, 2, 3, 4, 5, 6],
                b: vec![6, 5, 4, 3, 2, 1],
                output: false,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.a, tc.b));
        }
    }
}
