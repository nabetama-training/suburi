pub struct Solution;

impl Solution {
    pub fn solve(matrix_str: String) -> String {
        let lines: Vec<&str> = matrix_str.lines().collect();
        let dimensions = lines[0].split_whitespace().collect::<Vec<&str>>();
        let rows = dimensions[0].parse::<usize>().unwrap(); // n
        let cols = dimensions[1].parse::<usize>().unwrap(); // m

        // 行列の読み込み
        let matrix: Vec<Vec<i32>> = lines[1..=rows]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num_str| num_str.to_i32())
                    .collect()
            })
            .collect();

        // ベクトルの読み込み
        let vector: Vec<i32> = lines[rows + 1..]
            .iter()
            .map(|num_str| num_str.to_i32())
            .collect();

        let mut result = vec![0; rows];
        for i in 0..rows {
            for j in 0..cols {
                result[i] += matrix[i][j] * vector[j];
            }
        }
        result
            .into_iter()
            .map(|num| num.to_string())
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
            matrix: String,
            output: String,
        }
        let test_cases = [TestCase {
            matrix: "3 4
1 2 0 1
0 3 0 1
4 1 1 0
1
2
3
0"
            .to_string(),
            output: "5\n6\n9".to_string(),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.matrix));
        }
    }
}
