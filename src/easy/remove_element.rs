pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;

    while i < nums.len() {
        // 削除対象だったら
        if nums[i] == val {
            // リストから消す（次の数が同じインデックスにスライドしてくるので次の数が探索できる）
            nums.remove(i);
        } else {
            // 削除対象じゃない場合は次の数を探索するためにインデックスをインクリメント
            i += 1;
        }
    }

    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        struct TestCase {
            nums: Vec<i32>,
            val: i32,
            expect: i32,
        }

        let test_cases = [
            TestCase {
                nums: vec![3, 2, 2, 3],
                val: 3,
                expect: 2,
            },
            TestCase {
                nums: vec![0, 1, 2, 2, 3, 0, 4, 2],
                val: 2,
                expect: 5,
            },
        ];

        for mut tc in test_cases {
            assert_eq!(tc.expect, remove_element(&mut tc.nums, tc.val));
        }
    }
}
