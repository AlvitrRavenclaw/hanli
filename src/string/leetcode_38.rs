#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_and_say(n: i32) -> String {
        let mut ret: String = "1".to_string();
        for _ in 1..n {
            let mut tmp = String::new();
            let mut cur: char = ' ';
            let mut cnt: u8 = 0;
            for ch in ret.chars() {
                if cur == ch {
                    cnt += 1;
                } else {
                    if cur != ' ' && cnt != 0 {
                        tmp.push_str(&cnt.to_string());
                        tmp.push(cur);
                    }
                    cur = ch;
                    cnt = 1;
                }
            }
            tmp.push_str(&cnt.to_string());
            tmp.push(cur);
            ret = tmp;
        }
        return ret;
    }
}

#[test]
fn test_count_and_say() {
    for n in 0..=30 {
        println!("{}", Solution::count_and_say(n));
    }
}
