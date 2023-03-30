use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn solve(_: i32, cards: String) -> String {
        let mut stocks = HashMap::new();

        let cards: Vec<&str> = cards.split(',').collect();
        for card in cards {
            let mut parts = card.split_whitespace();
            let s = parts.next().unwrap().to_string();
            let rank = parts.next().unwrap().to_string();
            let key = format!("{s} {rank}");
            stocks.insert(key, true);
        }

        let mut result = String::new();

        // 出力の制約
        // 絵柄がスペード、ハート、クラブ、ダイヤの順番で優先的に出力する
        for suit in &["S", "H", "C", "D"] {
            // 絵柄が同じ場合は、ランクが小さい順に出力する
            for rank in 1..14 {
                let key = format!("{suit} {rank}");
                if stocks.contains_key(&key) {
                    continue;
                } else {
                    result.push_str(&key);
                    result.push(',');
                }
            }
        }
        result.pop();
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            n: i32,
            cards: String,
            output: String,
        }
        let test_cases = [TestCase {
            n: 47,
            // コードが縦に長くなるため、便宜上改行ではなく comma にする。アルゴリズムは同じ
            cards: "S 10,S 11,S 12,S 13,H 1,H 2,S 6,S 7,S 8,S 9,H 6,H 8,H 9,H 10,H 11,H 4,H 5,S 2,S 3,S 4,S 5,H 12,H 13,C 1,C 2,D 1,D 2,D 3,D 4,D 5,D 6,D 7,C 3,C 4,C 5,C 6,C 7,C 8,C 9,C 10,C 11,C 13,D 9,D 10,D 11,D 12,D 13".to_string(),
            output: "S 1,H 3,H 7,C 12,D 8".to_string(),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.n, tc.cards));
        }
    }
}
