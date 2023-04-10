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

    // pub fn south(&self) -> Dice {
    //     Dice {
    //         one: self.five,
    //         two: self.one,
    //         three: self.three,
    //         four: self.four,
    //         five: self.six,
    //         six: self.two,
    //     }
    // }

    // pub fn east(&self) -> Dice {
    //     Dice {
    //         one: self.four,
    //         two: self.two,
    //         three: self.one,
    //         four: self.six,
    //         five: self.five,
    //         six: self.three,
    //     }
    // }

    // pub fn west(&self) -> Dice {
    //     Dice {
    //         one: self.three,
    //         two: self.two,
    //         three: self.six,
    //         four: self.one,
    //         five: self.five,
    //         six: self.four,
    //     }
    // }
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
        // up が上にくるまで縦回転させる
        while dice.one != up {
            dice = dice.north();
        }
        // front が前にくるまで横回転させる
        while dice.two != front {
            dice = dice.right();
        }
        // right
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
        let test_cases = [TestCase {
            dice: vec![1, 2, 3, 4, 5, 6],
            up: 6,
            front: 5,
            output: 3,
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.dice, tc.up, tc.front));
        }
    }
}
