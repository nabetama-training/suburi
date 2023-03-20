pub struct Solution;

impl Solution {
    // sort した新しい配列を返すのではなく, nums1 に merge する
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        struct TestCase {
            nums1: Vec<i32>,
            m: i32,
            nums2: Vec<i32>,
            n: i32,
            expect: Vec<i32>,
        }
        let test_cases = [
            TestCase {
                nums1: vec![1, 2, 3, 0, 0, 0],
                m: 3,
                nums2: vec![2, 5, 6],
                n: 3,
                expect: vec![1, 2, 2, 3, 5, 6],
            },
            TestCase {
                nums1: vec![1],
                m: 1,
                nums2: vec![],
                n: 0,
                expect: vec![1],
            },
        ];
        for mut tc in test_cases {
            Solution::merge(&mut tc.nums1, tc.m, &mut tc.nums2, tc.n);
            assert_eq!(tc.nums1, tc.expect);
        }
    }
}
