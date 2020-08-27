#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ret = String::new();
        if strs.len() == 0 {
            return ret;
        }
        
        let mut col = 0;
        loop {
            let mut cur = ' ';
            let mut cnt = 0;
            for s in strs.iter() {
                if col == s.len() {
                    return ret;
                }
                let ch = s.chars().nth(col).unwrap();
                if cur == ch {
                    cnt += 1;
                } else {
                    if cur != ' ' {
                        return ret;
                    } else {
                        cur = ch;
                        cnt += 1;
                    }
                }
            }
            if cnt == strs.len() {
                ret.push(cur);
            }
            col += 1;
        }
    }
}

#[test]
fn test_longest_common_prefix() {
    println!(
        "[{}]",
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ])
    );
    println!(
        "[{}]",
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ])
    );
}
