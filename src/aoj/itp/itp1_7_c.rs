// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_C&lang=ja
pub struct Solution;

impl Solution {
    pub fn solve(r: i32, c: i32, matrix_str: String) -> String {
        let lines = matrix_str.lines().collect::<Vec<&str>>();

        let mut matrix = lines[0..r as usize]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num_str| num_str.to_i32())
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>();

        // col の sum を 0 で初期化して pushする
        matrix.push(vec![0; c as usize + 1]);

        for i in 0..r {
            let mut row_sum = 0;
            for j in 0..c {
                row_sum += matrix[i as usize][j as usize];
                // col の sum を算出
                matrix[r as usize][j as usize] += matrix[i as usize][j as usize];
            }
            matrix[i as usize].push(row_sum);
        }

        // 最後に col_sum の sum を算出
        matrix[r as usize][c as usize] = matrix[r as usize][0..c as usize].iter().sum();

        // [[1, 1, 3, 4, 5, 14], [2, 2, 2, 4, 5, 15], [3, 3, 0, 1, 1, 8], [2, 3, 4, 4, 6, 19], [8, 9, 9, 13, 17, 56]]
        // println!("{matrix:?}");

        matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

trait I32er {
    fn to_i32(&self) -> i32;
}

impl I32er for &str {
    fn to_i32(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        struct TestCase {
            r: i32,
            c: i32,
            matrix_str: String,
            output: String,
        }
        let test_cases = [TestCase {
            r: 4,
            c: 5,
            matrix_str: "1 1 3 4 5
2 2 2 4 5
3 3 0 1 1
2 3 4 4 6"
                .to_string(),
            output: "1 1 3 4 5 14
2 2 2 4 5 15
3 3 0 1 1 8
2 3 4 4 6 19
8 9 9 13 17 56"
                .to_string(),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.r, tc.c, tc.matrix_str));
        }
    }
}
