#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse(x: i32) -> i32 {
        let mut num = (x as i64).abs();
        if num < 10 {
            return x;
        }
        let mut ret: i64 = 0;
        while num != 0 {
            let rem = num % 10;
            ret = ret * 10 + (rem as i64);
            num /= 10;
        }
        if x < 0 {
            return if ret <= 2147483648 {-ret as i32} else {0}
        } else {
            return if ret <= 2147483647 {ret as i32} else {0}
        }
    }
}

#[test]
fn test_reverse() {
    println!("{:?}", Solution::reverse(123));
    println!("{:?}", Solution::reverse(-123));
    println!("{:?}", Solution::reverse(120));
    println!("{:?}", Solution::reverse(-2147483648));
}
