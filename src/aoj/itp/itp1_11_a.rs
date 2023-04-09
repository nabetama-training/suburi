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
}

pub struct Solution;

impl Solution {
    pub fn solve(dice: Vec<i32>, order: String) -> i32 {
        let mut dice = Dice {
            one: dice[0],
            two: dice[1],
            three: dice[2],
            four: dice[3],
            five: dice[4],
            six: dice[5],
        };
        for c in order.chars() {
            match c {
                'N' => dice = dice.north(),
                'S' => dice = dice.south(),
                'E' => dice = dice.east(),
                'W' => dice = dice.west(),
                _ => panic!("invalid order"),
            }
        }
        dice.one
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            dice: Vec<i32>,
            order: String,
            output: i32,
        }
        let test_cases = [
            TestCase {
                dice: vec![1, 2, 4, 8, 16, 32],
                order: "SE".to_string(),
                output: 8,
            },
            TestCase {
                dice: vec![1, 2, 4, 8, 16, 32],
                order: "EESWN".to_string(),
                output: 32,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.dice, tc.order));
        }
    }
}
