pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
    // using only sorted array
    nums.dedup();
    nums.len() as i32
}

pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    let mut duplicate_count = 0;

    for i in 1..nums.len() {
        // 次の数が一つ前の数と等しければ
        if nums[i] == nums[i - 1] {
            // 重複回数をインクリメントする
            duplicate_count += 1;
        }
        // ユニークになっている数の次の位置に挿入する
        nums[i - duplicate_count] = nums[i];
    }

    // 全体から重複した回数をマイナス → ユニークになった数字の個数
    (nums.len() - duplicate_count) as i32
}

pub fn remove_duplicates3(nums: &mut Vec<i32>) -> i32 {
    let mut prev_number: Option<i32> = None; // 前回の数値
    let mut i = 0;

    // nums.len() は動的に変化する
    while i < nums.len() {
        if prev_number == Some(nums[i]) {
            nums.remove(i); // 前回の数値と同じ場合、今回の要素を削除する
        } else {
            prev_number = Some(nums[i]); // 前回の数値と異なっていれば、配列操作は行わず、前回の数値（prev_number）を更新する
            i += 1;
        }
    }

    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        output: i32,
    }

    #[test]
    fn test_remove_duplicates() {
        let test_cases = [
            TestCase {
                input: vec![1, 1, 2],
                output: 2,
            },
            TestCase {
                input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
                output: 5,
            },
            TestCase {
                input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5],
                output: 6,
            },
        ];

        for mut tc in test_cases {
            // assert_eq!(tc.output, remove_duplicates(&mut tc.input));
            assert_eq!(tc.output, remove_duplicates(&mut tc.input));
        }
    }

    #[test]
    fn test_remove_duplicates2() {
        let test_cases = [
            TestCase {
                input: vec![1, 1, 2],
                output: 2,
            },
            TestCase {
                input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
                output: 5,
            },
            TestCase {
                input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5],
                output: 6,
            },
        ];

        for mut tc in test_cases {
            // assert_eq!(tc.output, remove_duplicates(&mut tc.input));
            assert_eq!(tc.output, remove_duplicates2(&mut tc.input));
        }
    }

    #[test]
    fn test_remove_duplicates3() {
        let test_cases = [
            TestCase {
                input: vec![1, 1, 2],
                output: 2,
            },
            TestCase {
                input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
                output: 5,
            },
            TestCase {
                input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5],
                output: 6,
            },
        ];

        for mut tc in test_cases {
            // assert_eq!(tc.output, remove_duplicates(&mut tc.input));
            assert_eq!(tc.output, remove_duplicates3(&mut tc.input));
        }
    }
}
