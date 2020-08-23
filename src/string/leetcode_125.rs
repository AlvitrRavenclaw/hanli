#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < j {
            if !chars[i].is_ascii_alphabetic() && !chars[i].is_ascii_digit() {
                i += 1;
                continue;
            }
            if !chars[j].is_ascii_alphabetic() && !chars[j].is_ascii_digit() {
                j -= 1;
                continue;
            }
            if chars[i].is_ascii_digit() && chars[j].is_ascii_digit() {
                if chars[i] != chars[j] {
                    return false
                }
                i += 1;
                j -= 1;
            } else if chars[i].is_alphabetic() && chars[j].is_alphabetic() {
                let ci = chars[i] as i16;
                let cj = chars[j] as i16;
                if ci - cj != 0 && (ci - cj).abs() != 32 {
                    return false;
                }
                i += 1;
                j -= 1;
            } else {
                return false;
            }

        }
        return true;
    }
}

#[test]
fn test_is_palindrome() {
    println!("{}", Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")));
    println!("{}", Solution::is_palindrome(String::from("race a car")));
}
