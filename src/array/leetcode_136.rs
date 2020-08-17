#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ret: i32 = 0;
        for x in nums {
            ret ^= x;
        }
        return ret;
    }
}

#[test]
fn test_single_number() {
    println!("{:?}", Solution::single_number(vec![2, 2, 1]));
    println!("{:?}", Solution::single_number(vec![4, 1, 2, 1, 2]));
}