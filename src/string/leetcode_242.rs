#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let n = s.len();
        let mut cnt: [i64; 26] = [0; 26];
        let schars: Vec<char> = s.chars().collect();
        let tchars: Vec<char> = t.chars().collect();
        for i in 0..n {
            let p = schars[i] as usize - 97;
            let q = tchars[i] as usize - 97;
            cnt[p] += 1;
            cnt[q] -= 1;
        }
        for i in 0..26 {
            if cnt[i] != 0 {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test_is_anagram() {
    println!("{}", Solution::is_anagram(String::from("anagram"), String::from("nagaram")));
    println!("{}", Solution::is_anagram(String::from("rat"), String::from("car")));
}
