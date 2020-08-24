#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        let h: Vec<char> = haystack.chars().collect();
        let n: Vec<char> = needle.chars().collect();
        for i in 0..h.len() {
            if i + n.len() > h.len() {
                return -1;
            }
            if h[i] != n[0] {
                continue;
            }
            let mut _match = 0; 
            for j in 0..n.len() {
                if i + n.len() > h.len() {
                    return -1;
                }
                if i + j < h.len() && h[i + j] == n[j] {
                    _match += 1;
                } else {
                    break;
                }
            }
            if _match == n.len() {
                return i as i32;
            }
        }
        return -1;
    }
}

#[test]
fn test_str_str() {
    println!("{}", Solution::str_str("hello".to_string(), "ll".to_string()));
    println!("{}", Solution::str_str("aaaaa".to_string(), "bba".to_string()));
}
