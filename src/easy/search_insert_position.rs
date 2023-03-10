use std::cmp::Ordering;

// Given a sorted array of distinct integers and a target value,
// return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    if target < *nums.first().unwrap() {
        return 0;
    }

    // binary serach
    let mut low = 0;
    let mut high = nums.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        match target.cmp(&nums[mid]) {
            Ordering::Equal => return mid as i32,
            Ordering::Greater => low = mid + 1,
            Ordering::Less => high = mid - 1,
        }
    }

    low as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_insert() {
        struct TestCase {
            nums: Vec<i32>,
            target: i32,
            output: i32,
        }

        let test_cases = [
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 5,
                output: 2,
            },
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 2,
                output: 1,
            },
            TestCase {
                nums: vec![1, 3, 5, 6],
                target: 7,
                output: 4,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.output, search_insert(tc.nums, tc.target));
        }
    }
}
