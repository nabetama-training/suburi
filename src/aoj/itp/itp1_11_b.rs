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
    #[allow(dead_code)]
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
}

pub struct Solution;

impl Solution {
    pub fn solve(dice: Vec<i32>, up: i32, front: i32) -> i32 {
        let mut dice = Dice {
            one: dice[0],
            two: dice[1],
            three: dice[2],
            four: dice[3],
            five: dice[4],
            six: dice[5],
        };

        // 上が up になるように回転させる(0 <= n <= 6)
        let patterns = vec!["", "N", "N", "N", "E", "EE"];

        for pattern in patterns {
            match pattern {
                "N" => dice = dice.north(),
                "E" => dice = dice.east(),
                "EE" => dice = dice.east().east(),
                _ => {}
            }
            if dice.one == up {
                break;
            }
        }

        // 横回転させて正面を洗い出す(0 <= n <= 4)
        while dice.two != front {
            dice = dice.right();
        }
        // three は右側面
        dice.three
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            dice: Vec<i32>,
            up: i32,
            front: i32,
            output: i32,
        }
        let test_cases = [
            TestCase {
                dice: vec![1, 2, 3, 4, 5, 6],
                up: 6,
                front: 5,
                output: 3,
            },
            TestCase {
                dice: vec![1, 2, 3, 4, 5, 6],
                up: 1,
                front: 3,
                output: 5,
            },
            TestCase {
                dice: vec![1, 2, 3, 4, 5, 6],
                up: 3,
                front: 2,
                output: 6,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.dice, tc.up, tc.front));
        }
    }
}
