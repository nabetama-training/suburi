#[allow(dead_code)]
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
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

#[test]
fn test_two_sum() {
    struct TestCase<T> {
        numbers: Vec<T>,
        target: i32,
        expect: Vec<i32>,
    }

    let test_cases = [TestCase {
        numbers: vec![2, 7, 11, 15],
        target: 9,
        expect: vec![0, 1],
    }];

    for tc in test_cases {
        assert_eq!(two_sum(tc.numbers, tc.target), tc.expect)
    }
}
