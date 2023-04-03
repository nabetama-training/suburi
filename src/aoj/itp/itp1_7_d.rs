pub struct Solution;

impl Solution {
    pub fn solve(matrix_str: String) -> String {
        let lines: Vec<&str> = matrix_str.lines().collect();
        let dimensions = lines[0].split_whitespace().collect::<Vec<&str>>();
        let n = dimensions[0].parse::<usize>().unwrap();
        let m = dimensions[1].parse::<usize>().unwrap();
        let l = dimensions[2].parse::<usize>().unwrap();

        let left_matrix = lines[1..=n]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num_str| num_str.to_i32())
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>();

        // [[1, 2], [0, 3], [4, 5]]
        // println!("{left_matrix:?}");

        let right_matrix = lines[n + 1..]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num_str| num_str.to_i32())
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>();

        // [[1, 2, 1], [0, 3, 2]]
        // println!("{right_matrix:?}");

        let mut result = vec![vec![0; l]; n];
        // [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
        // println!("{result:?}");

        // n = 3, l = 3, m = 2
        for i in 0..n {
            for j in 0..l {
                #[allow(clippy::needless_range_loop)]
                for k in 0..m {
                    result[i][j] += left_matrix[i][k] * right_matrix[k][j];
                }
            }
        }

        println!("{result:?}");

        result
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
            matrix_str: String,
            output: String,
        }
        let test_cases = [TestCase {
            matrix_str: "3 2 3\n1 2\n0 3\n4 5\n1 2 1\n0 3 2".to_string(),
            output: "1 8 5\n0 9 6\n4 23 14".to_string(),
        }];
        for tc in test_cases {
            assert_eq!(tc.output, Solution::solve(tc.matrix_str));
        }
    }
}
