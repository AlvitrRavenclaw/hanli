#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        let k = (s.len() / 2) as usize;
        for i in 0..k {
            s.swap(i, n - i - 1)
        }
    }
}

#[test]
fn test_reverse_string() {
    let mut case0 = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut case0);
    println!("{:?}", case0);
}
