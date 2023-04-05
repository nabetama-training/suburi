pub struct Solution;

impl Solution {
    const WIN_PT: i32 = 3;
    const DRAW_PT: i32 = 1;

    pub fn solve(cards: Vec<(String, String)>) -> (i32, i32) {
        let mut taro_score = 0;
        let mut hanako_score = 0;

        for (taro_card, hanako_card) in cards {
            match taro_card.cmp(&hanako_card) {
                std::cmp::Ordering::Less => hanako_score += Self::WIN_PT,
                std::cmp::Ordering::Equal => {
                    taro_score += Self::DRAW_PT;
                    hanako_score += Self::DRAW_PT;
                }
                std::cmp::Ordering::Greater => taro_score += Self::WIN_PT,
            }
        }
        (taro_score, hanako_score)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            cards: Vec<(String, String)>,
            output: (i32, i32),
        }
        let test_cases = [
            TestCase {
                cards: vec![
                    ("cat".to_string(), "dog".to_string()),
                    ("fish".to_string(), "fish".to_string()),
                    ("lion".to_string(), "tiger".to_string()),
                ],
                output: (1, 7),
            },
            TestCase {
                cards: vec![
                    ("cat".to_string(), "dog".to_string()),
                    ("fish".to_string(), "fish".to_string()),
                    ("find".to_string(), "fish".to_string()),
                    ("books".to_string(), "book".to_string()),
                    ("lion".to_string(), "tiger".to_string()),
                ],
                output: (4, 10),
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.cards));
        }
    }
}
