#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cnt: [u64; 26] = [0; 26];
        let chars: Vec<char> = s.chars().collect();
        for ch in chars.iter() {
            cnt[*ch as usize - 97] += 1;
        }
        for (i, ch) in chars.iter().enumerate() {
            if cnt[*ch as usize - 97] == 1 {
                return i as i32;
            }
        }
        return -1;
    }
}

#[test]
fn test_first_uniq_char() {
    println!("{}", Solution::first_uniq_char(String::from("leetcode")));
    println!("{}", Solution::first_uniq_char(String::from("loveleetcode")));
}
