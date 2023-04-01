pub struct Solution;

impl Solution {
    pub fn solve(_: i32, info: String) -> String {
        let lines: Vec<&str> = info.split('\n').collect();

        let mut counter = vec![vec![vec![0; 10]; 3]; 4];

        for line in lines {
            let mut bfrv = line.split_whitespace();
            let b = bfrv.next().unwrap().to_string();
            let f = bfrv.next().unwrap().to_string();
            let r = bfrv.next().unwrap().to_string();
            let v = bfrv.next().unwrap().to_string();

            counter[b.to_usize() - 1][f.to_usize() - 1][r.to_usize() - 1] += v.to_usize() as i32;
        }

        let mut result = String::new();

        for (v, b) in counter.iter().zip(0..4) {
            for rooms in v {
                for room in rooms {
                    print!(" {room}");
                    result.push_str(&format!(" {}", &room.to_string()));
                }
                // println!();
                result.push('\n');
            }

            if b < 3 {
                // println!("{}", "#".repeat(20));
                result.push_str(&format!("{}\n", &"#".repeat(20)));
            }
        }

        // println!("{counter:?}");
        result
    }
}

trait Numer {
    fn to_usize(&self) -> usize;
}

impl Numer for String {
    fn to_usize(&self) -> usize {
        let n: i32 = self.parse().unwrap();
        n as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_func() {
        let answer = " 0 0 8 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0
####################
 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0
####################
 0 0 0 0 0 0 0 0 0 0
 0 7 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0
####################
 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 1 0 0
";

        assert_eq!(
            answer.to_string(),
            Solution::solve(4, "1 1 3 8\n3 2 2 7\n4 3 8 1".to_string()),
        )
    }
}
