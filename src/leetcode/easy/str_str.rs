//! 28. Find the Index of the First Occurrence in a String
//! Given two strings needle and haystack,
//! return the index of the first occurrence of needle in haystack,
//! or -1 if needle is not part of haystack.

// algorithm
pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() < needle.len() {
        return -1;
    }

    // needleが1文字のときもあるので +1 しておく
    for i in 0..(haystack.len() - needle.len() + 1) {
        // 最初の1文字目からあneedle.len() までの文字列が needle に等しければ
        if haystack[i..i + needle.len()] == needle {
            // 最初に見つかったインデックスを返す
            return i as i32;
        }
    }
    -1
}

/// use core::str::find
pub fn str_str2(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        index as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_str() {
        struct TestCase {
            haystack: String,
            needle: String,
            expect: i32,
        }

        let test_cases = [
            TestCase {
                haystack: "sadbutsad".to_string(),
                needle: "sad".to_string(),
                expect: 0,
            },
            TestCase {
                haystack: "leetcode".to_string(),
                needle: "leeto".to_string(),
                expect: -1, // "leeto" did not occur in "leetcode", so we return -1
            },
            TestCase {
                haystack: "abb".to_string(),
                needle: "abaaa".to_string(),
                expect: -1, // "leeto" did not occur in "leetcode", so we return -1
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expect, str_str(tc.haystack, tc.needle));
        }
    }

    #[test]
    fn test_str_str2() {
        struct TestCase {
            haystack: String,
            needle: String,
            expect: i32,
        }

        let test_cases = [
            TestCase {
                haystack: "sadbutsad".to_string(),
                needle: "sad".to_string(),
                expect: 0,
            },
            TestCase {
                haystack: "leetcode".to_string(),
                needle: "leeto".to_string(),
                expect: -1, // "leeto" did not occur in "leetcode", so we return -1
            },
            TestCase {
                haystack: "abb".to_string(),
                needle: "abaaa".to_string(),
                expect: -1, // "leeto" did not occur in "leetcode", so we return -1
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expect, str_str2(tc.haystack, tc.needle));
        }
    }
}
