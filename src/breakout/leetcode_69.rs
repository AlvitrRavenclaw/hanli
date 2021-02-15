struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }
        let (mut i, mut j) = (1, (x / 2) + 1);
        while i <= j {
            let m = i + (j - i) / 2;
            if x / m == m {
                return m
            } else if x / m > m {
                i = m + 1
            } else {
                j = m - 1
            }
        }
        return j;
    }
}

#[test]
fn test_my_sqrt() {
    println!("{}", Solution::my_sqrt(5));
    println!("{}", Solution::my_sqrt(8));
}