#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_atoi(str: String) -> i32 {
        let s = str.trim();
        if s.len() == 0 {
            return 0;
        }
        let chars: Vec<char> = s.chars().collect();
        
        let mut start = 0;
        if !chars[0].is_ascii_digit() && chars[0] != '+' && chars[0] != '-' {
            return 0
        } else {
            start = if chars[0].is_ascii_digit() {0} else {1};
        }
        
        let mut ret: u64 = 0; 
        for i in start..chars.len() {
            if !chars[i].is_ascii_digit() || ret > 2147483647 {
                break;
            }
            let tmp = chars[i] as u64 - 48;
            ret = ret * 10 + tmp;
        }

        if chars[0] != '-' {
            return if ret > 2147483647 { 2147483647i32 } else { ret as i32 };
        } else {
            return if ret > 2147483648 { -2147483648i32 } else { -(ret as i32) }
        }
    }
}

#[test]
fn test_my_atoi() {
    println!("{}", Solution::my_atoi("42".to_string()));
    println!("{}", Solution::my_atoi("   -42".to_string()));
    println!("{}", Solution::my_atoi("4193 with words".to_string()));
    println!("{}", Solution::my_atoi("words and 987".to_string()));
    println!("{}", Solution::my_atoi("-91283472332".to_string()));
    println!("{}", Solution::my_atoi("91283472332".to_string()));
    println!("{}", Solution::my_atoi("9223372036854775808".to_string()));
    println!("{}", Solution::my_atoi("18446744073709551617".to_string()));
}
