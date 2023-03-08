pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            if numbers[i] + numbers[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![1, 2, 3]
}

pub fn two_sum_use_hash_map(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut hm = HashMap::<i32, i32>::new();

    for (pos, num) in numbers.iter().enumerate() {
        let sub = target - num;

        match hm.get(&sub) {
            Some(&val) => {
                return vec![val, pos as i32];
            }
            None => {
                hm.insert(*num, pos as i32);
            }
        }
    }
    panic!("solution is nothhing")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        #[derive(Debug)]
        struct TestCase {
            numbers: Vec<i32>,
            target: i32,
            expect: Vec<i32>,
        }

        let test_cases = [
            TestCase {
                numbers: vec![2, 7, 11, 15],
                target: 9,
                expect: vec![0, 1],
            },
            TestCase {
                numbers: vec![2, 7, 4, 15],
                target: 11,
                expect: vec![1, 2],
            },
            TestCase {
                numbers: vec![2, 7, 4, 15],
                target: 19,
                expect: vec![2, 3],
            },
        ];

        for tc in test_cases {
            assert_eq!(two_sum(tc.numbers, tc.target), tc.expect);
        }
    }

    #[test]
    fn test_two_sum_use_hash_map() {
        #[derive(Debug)]
        struct TestCase {
            numbers: Vec<i32>,
            target: i32,
            expect: Vec<i32>,
        }

        let test_cases = [
            TestCase {
                numbers: vec![2, 7, 11, 15],
                target: 9,
                expect: vec![0, 1],
            },
            TestCase {
                numbers: vec![2, 7, 4, 15],
                target: 11,
                expect: vec![1, 2],
            },
            TestCase {
                numbers: vec![2, 7, 4, 15],
                target: 19,
                expect: vec![2, 3],
            },
        ];

        for tc in test_cases {
            assert_eq!(two_sum_use_hash_map(tc.numbers, tc.target), tc.expect);
        }
    }
}
